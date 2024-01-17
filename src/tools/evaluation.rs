use {
    std::{collections::HashMap, path::Path, fs},
    tracing::info,
    serde::{Serialize, Deserialize},
    rand::seq::SliceRandom,
    crate::{
        utils::{metrics::init_metrics, generate_run_id},
        client::{ReplacementPolicy, RandomReplacementPolicy, LeastRecentlyUsedReplacementPolicy, MostRecentlyUsedReplacementPolicy, PreferRemoteSpansReplacementPolicy},
        demo::{
            llm_inference::run_llm_inference_demo,
            web_service::run_web_service_demo,
            dataframe::run_dataframe_demo,
        },
    },
};

#[derive(Serialize, Deserialize)]
pub struct EvaluationData {
    values: HashMap<String, f32>,
}

impl EvaluationData {
    pub fn get_experiment_result(&self, experiment: &Experiment) -> Option<f32> {
        self.values.get(&experiment.get_key()).cloned()
    }
}

#[derive(Debug)]
pub struct Experiment {
    pub local_memory_percent: u32,
    pub application: DemoApplicationType,
    pub zipf_s: Option<u32>, // 0..100
    pub span_replacement_policy: Option<SpanReplacementPolicy>,
}

impl Experiment {
    pub fn get_key(&self) -> String {
        let mut res = format!("v2_local_{}_application_{}", self.local_memory_percent, self.application.get_key());

        if let Some(zipf_s) = self.zipf_s {
            res = format!("{}_zipf_{}", res, zipf_s);
        }

        if let Some(replacement_policy) = &self.span_replacement_policy {
            if replacement_policy != &SpanReplacementPolicy::Replay {
                res = format!("{}_replacement_{}", res, replacement_policy.get_key());
            }
        }

        res
    }
}

#[derive(Debug, Clone)]
pub enum DemoApplicationType {
    LlmInference,
    WebService,
    Dataframe,
}

impl DemoApplicationType {
    pub fn total_memory(&self) -> u32 {
        match self {
            Self::LlmInference => 25710,
            Self::WebService => 8799,
            Self::Dataframe => 9485,
        }
    }

    pub fn get_key(&self) -> String {
        match self {
            Self::LlmInference => "llm_inference",
            Self::WebService => "web_service_v2",
            Self::Dataframe => "dataframe_v3_30m",
        }.to_owned()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SpanReplacementPolicy {
    Replay,
    LRU,
    MRU,
    Random,
    PreferRemoteLRU,
    PreferRemoteMRU,
}

impl SpanReplacementPolicy {
    pub fn get_key(&self) -> String {
        match self {
            Self::Replay => "replay",
            Self::LRU => "lru_v2",
            Self::MRU => "mru",
            Self::Random => "random",
            Self::PreferRemoteLRU => "prefer_remote_lru",
            Self::PreferRemoteMRU => "prefer_remote_mru",
        }.to_owned()
    }
}

pub fn run_evaluation(storage_endpoint: String, manager_endpoint: String) {
    info!("running evaluation");

    let granularity: u32 = 10;

    let evaluation_data = load_evaluation_data();

    let mut experiments: Vec<Experiment> = vec![];

    // plot with throughput per application and memory usage.
    for application in [DemoApplicationType::LlmInference, DemoApplicationType::WebService, DemoApplicationType::Dataframe] {
        for local_memory_percent in (granularity..=100).step_by(granularity as usize) {
            experiments.push(Experiment {
                local_memory_percent,
                application: application.clone(),
                zipf_s: None,
                span_replacement_policy: None,
            });
        }
    }

    // plot different distributions
    for zipf_s in (0..=100).step_by(granularity as usize) {
        experiments.push(Experiment {
            local_memory_percent: 80,
            application: DemoApplicationType::WebService,
            zipf_s: Some(zipf_s),
            span_replacement_policy: None,
        })
    }

    // test different repacement policies
    for span_replacement_policy in [
        SpanReplacementPolicy::Replay,
        SpanReplacementPolicy::LRU,
        // SpanReplacementPolicy::MRU,
        SpanReplacementPolicy::Random,
        // SpanReplacementPolicy::PreferRemoteLRU,
        // SpanReplacementPolicy::PreferRemoteMRU,
    ] {
        for local_memory_percent in (granularity..=100).step_by(granularity as usize) {
            experiments.push(Experiment {
                local_memory_percent,
                application: DemoApplicationType::LlmInference,
                zipf_s: None,
                span_replacement_policy: Some(span_replacement_policy.clone()),
            });
        }
    }

    info!("total {} experiments", experiments.len());
    let experiments: Vec<_> = experiments.into_iter()
        .filter(|exp| !evaluation_data.values.contains_key(&exp.get_key()))
        .collect();

    info!("experiments remaining to run: {}", experiments.len());
    if experiments.is_empty() {
        info!("no experiments left to run");
        return;
    }

    let experiment = experiments.choose(&mut rand::thread_rng()).unwrap();

    let result = run_experiment(&experiment, storage_endpoint, manager_endpoint);
    let evaluation_data = {
        let mut evaluation_data = evaluation_data;
        evaluation_data.values.insert(experiment.get_key(), result);
        evaluation_data
    };
    save_evaluation_data(evaluation_data);
}

fn run_experiment(experiment: &Experiment, storage_endpoint: String, manager_endpoint: String) -> f32 {
    info!("running experiment: {:?}", experiment);

    let run_id = generate_run_id();
    let metrics = init_metrics(None);

    let memory_limit = if experiment.local_memory_percent == 100  {
        None
    } else {
        Some((experiment.application.total_memory() as f32 * experiment.local_memory_percent as f32 / 100.0) as u64)
    }.map(|v| v * 1024 * 1024);

    let token = read_token();
    let storage_endpoints = storage_endpoint.split(",").map(|v| v.to_owned()).collect::<Vec<_>>();

    let replacement_policy: Option<Box<dyn ReplacementPolicy>> = if let Some(policy_type) = &experiment.span_replacement_policy {
        match policy_type {
            SpanReplacementPolicy::Replay => None,
            SpanReplacementPolicy::LRU => Some(Box::new(LeastRecentlyUsedReplacementPolicy::new())),
            SpanReplacementPolicy::MRU => Some(Box::new(MostRecentlyUsedReplacementPolicy::new())),
            SpanReplacementPolicy::PreferRemoteLRU => Some(Box::new(PreferRemoteSpansReplacementPolicy::new(Box::new(LeastRecentlyUsedReplacementPolicy::new())))),
            SpanReplacementPolicy::PreferRemoteMRU => Some(Box::new(PreferRemoteSpansReplacementPolicy::new(Box::new(MostRecentlyUsedReplacementPolicy::new())))),
            SpanReplacementPolicy::Random => Some(Box::new(RandomReplacementPolicy::new())),
        }
    } else {
        None
    };

    match experiment.application {
        DemoApplicationType::LlmInference => run_llm_inference_demo(
            metrics.clone(),
            run_id.clone(),
            &token,
            storage_endpoints,
            Some(manager_endpoint),
            10 * 60,
            false,
            memory_limit,
            replacement_policy,
        ),
        DemoApplicationType::WebService => run_web_service_demo(
            metrics.clone(),
            run_id.clone(),
            &token,
            storage_endpoints,
            Some(manager_endpoint),
            memory_limit,
            experiment.zipf_s.map(|v| (v as f32 / 100.0))
        ),
        DemoApplicationType::Dataframe => run_dataframe_demo(
            metrics.clone(),
            run_id.clone(),
            &token,
            storage_endpoints,
            Some(manager_endpoint),
            memory_limit,
        ),
    }
}

pub fn load_evaluation_data() -> EvaluationData {
    let path = "./evaluation.json";
    if !Path::new(&path).exists() {
        return EvaluationData {
            values: HashMap::new(),
        };
    }

    serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap()
}

fn save_evaluation_data(evaluation_data: EvaluationData) {
    std::fs::write("./evaluation.json", serde_json::to_vec(&evaluation_data).unwrap()).unwrap();
}

fn read_token() -> String {
    fs::read_to_string("config/.token").unwrap().replace("\n", "")
}
