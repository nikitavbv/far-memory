use {
    std::fs::File,
    tracing::{info, warn},
    prometheus::Registry,
    serde::{Serialize, Deserialize},
    chrono::{DateTime, Utc},
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
            FarMemorySerializedObjectVec,
        },
        manager::ManagerClient,
    },
};

#[derive(Serialize, Deserialize, Debug)]
struct FlightData {
    flight_date: DateTime<Utc>,
    airline: String,
    origin: String,
    destination: String,
    cancelled: bool,
    diverted: bool,
    crs_dep_time: u16,
    dep_time: f32,
    dep_delay_minutes: Option<f32>,
    dep_delay: Option<f32>,
    arr_time: f32,
    arr_delay_minutes: Option<f32>,
    air_time: f32,
    crs_elapsed_time: Option<f32>,
    actual_elapsed_time: f32,
    distance: f32,
    year: u32,
    quarter: u8,
    month: u8,
    day_of_month: u8,
    day_of_week: u8,
    marketing_airline_network: String,
    operated_or_branded_code_share_partners: String,
    dot_id_marketing_airline: u32,
    iata_code_marketing_airline: String,
    flight_number_marketing_airline: u32,
    operating_airline: String,
    dot_id_operating_airline: u32,
    iata_code_operating_airline: String,
    tail_number: String,
    flight_number_operating_airline: u32,
    origin_airport_id: u32,
    origin_airport_seq_id: u32,
    origin_city_market_id: u32,
    origin_city_name: String,
    origin_city_state: String,
    origin_state_fips: u32,
    origin_state_name: String,
    origin_wac: u32,
    dest_airport_id: u32,
    dest_airport_seq_id: u32,
    dest_city_market_id: u32,
    dest_city_name: String,
    dest_state: String,
    dest_state_fips: u32,
    dest_state_name: String,
    dest_wac: u32,
    dest_del_15: Option<f32>,
    departure_delay_groups: f32,
    dep_time_blk: String,
    taxi_out: f32,
    wheels_off: f32,
    wheels_on: f32,
    taxi_in: f32,
    crs_arr_time: u32,
    arr_delay: Option<f32>,
    arr_del_15: Option<f32>,
    arrival_delay_groups: Option<f32>,
    arr_time_blk: String,
    distance_group: u32,
    div_airport_landings: f32,
}

/**
This demo is designed to rely on streaming to demonstrate optimizations that are available in
such case.

using this dataset: https://www.kaggle.com/datasets/robikscube/flight-delay-dataset-20182022/
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
    let dataframe: FarMemorySerializedObjectVec<FlightData> = FarMemorySerializedObjectVec::new(client.clone());

    for year in 2018..2023 {
        let file_name = format!("./data/flights/Combined_Flights_{}.csv", year);
        let mut reader = csv::Reader::from_reader(File::open(file_name).unwrap());
        for row in reader.records() {
            let row = row.unwrap();
            println!("result: {:?}", row);
            panic!("done");
        }
    }

    // TODO: read dataset and load it into far memory
}
