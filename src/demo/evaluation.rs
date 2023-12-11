use {
    std::{collections::HashMap, path::Path, fs},
    tracing::info,
    serde::{Serialize, Deserialize},
    crate::utils::{metrics::init_metrics, generate_run_id},
    super::llm_inference::run_llm_inference_demo,
};

#[derive(Serialize, Deserialize)]
struct EvaluationData {
    values: HashMap<String, f32>,
}

#[derive(Debug)]
struct Experiment {
    local_memory_percent: u32,
}

impl Experiment {
    pub fn get_key(&self) -> String {
        format!("local_{}", self.local_memory_percent)
    }
}

pub fn run_evaluation(storage_endpoint: String, manager_endpoint: String) {
    info!("running evaluation");

    let evaluation_data = load_evaluation_data();

    let mut experiments: Vec<Experiment> = vec![];
    for local_memory_percent in (10..=100).step_by(10) {
        experiments.push(Experiment {
            local_memory_percent,
        });
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

    let experiment = experiments.into_iter().next().unwrap();

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

    let memory_limit_mb = if experiment.local_memory_percent == 100  {
        None
    } else {
        Some((25710 as f32 * experiment.local_memory_percent as f32 / 100.0) as u64)
    };

    run_llm_inference_demo(
        metrics.clone(),
        run_id.clone(),
        &read_token(),
        storage_endpoint.split(",").map(|v| v.to_owned()).collect::<Vec<_>>(),
        Some(manager_endpoint),
        10 * 60,
        false,
        memory_limit_mb.map(|v| v * 1024 * 1024)
    )
}

fn load_evaluation_data() -> EvaluationData {
    let path = "./data/evaluation.json";
    if !Path::new(&path).exists() {
        return EvaluationData {
            values: HashMap::new(),
        };
    }

    serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap()
}

fn save_evaluation_data(evaluation_data: EvaluationData) {
    std::fs::write("./data/evaluation.json", serde_json::to_vec(&evaluation_data).unwrap()).unwrap();
}

fn read_token() -> String {
    fs::read_to_string("config/.token").unwrap().replace("\n", "")
}
