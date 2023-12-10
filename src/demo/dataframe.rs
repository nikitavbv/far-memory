use {
    tracing::{info, warn},
    prometheus::Registry,
    crate::{
        client::{
            FarMemoryClient,
            FarMemoryBackend,
            NetworkNodeBackend,
            ErasureCodingBackend,
            ReplicationBackend,
            LocalDiskBackend,
            InstrumentedBackend,
            PreferRemoteSpansReplacementPolicy,
            MostRecentlyUsedReplacementPolicy,
            RemoteReplayReplacementPolicy,
        },
        manager::ManagerClient,
    },
};

/**
This demo is designed to rely on streaming to demonstrate optimizations that are available in
such case.
*/
pub fn run_dataframe_demo(metrics: Registry, run_id: String, token: &str, storage_endpoints: Vec<String>, manager_endpoint: Option<String>, local_max_memory: Option<u64>) {
    info!("running dataframe demo");

    // far memory client init
    let local_max_memory = local_max_memory.unwrap_or(20 * 1024 * 1024 * 1024);

    let manager_client = manager_endpoint.map(|endpoint| {
        let mut client = ManagerClient::new(&endpoint);
        client.auth(token);
        client
    });

    let backend: Box<dyn FarMemoryBackend> = if !storage_endpoints.is_empty() {
        if storage_endpoints.len() == 1 {
            info!("running in single backend node mode");
            Box::new(NetworkNodeBackend::new(&storage_endpoints[0], token, run_id))
        } else if storage_endpoints.len() == 5 {
            info!("running in erasure coded mode");

            let nodes: Vec<_> = storage_endpoints.iter()
                .map(|v| Box::new(NetworkNodeBackend::new(&v, token, run_id.clone())) as Box<dyn FarMemoryBackend>)
                .collect();

            Box::new(ErasureCodingBackend::new(nodes))
        } else {
            let nodes: Vec<_> = storage_endpoints.iter()
                .map(|v| Box::new(NetworkNodeBackend::new(&v, token, run_id.clone())) as Box<dyn FarMemoryBackend>)
                .collect();

            info!("running in replication mode with {} nodes", nodes.len());

            Box::new(ReplicationBackend::new(nodes))
        }
    } else {
        warn!("no storage endpoint provided, falling back to disk backend");
        Box::new(LocalDiskBackend::new())
    };

    let backend = Box::new(InstrumentedBackend::new(metrics.clone(), backend));
    let mut client = FarMemoryClient::new(backend, local_max_memory);
    if let Some(manager) = manager_client {
        let fallback = PreferRemoteSpansReplacementPolicy::new(Box::new(MostRecentlyUsedReplacementPolicy::new()));

        client.use_replacement_policy(Box::new(RemoteReplayReplacementPolicy::new(manager.clone(), Box::new(fallback))));
        client.use_manager(manager);
    }
    client.track_metrics(metrics.clone());
    client.start_swap_out_thread();

    // demo app
    // TODO: read dataset and load it into far memory
}
