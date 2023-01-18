use {
    std::{env, fs::File, io::Read, fmt::Write, time::{Duration, SystemTime, UNIX_EPOCH}},
    fuzzyhash::FuzzyHash,
    indicatif::{ProgressStyle, ProgressState},
    rdkafka::{ClientConfig, producer::{FutureProducer, FutureRecord}},
    serde::{Serialize, Deserialize},
    rand::Rng,
};

#[derive(Serialize, Deserialize)]
struct MemoryStateMessage {
    service: String,
    time: f64,
    changes: Vec<u32>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let producer = kafka_producer(&env::var("KAFKA_ENDPOINT").unwrap());
    
    let mut hashes = compute_swapfile_hashes();
    let mut memory_changes_tracked = 0;

    loop {
        let new_hashes = compute_swapfile_hashes();
        let changes = hashes_compare(&hashes, &new_hashes);
        hashes = new_hashes;

        let msg = MemoryStateMessage {
            service: "demo".to_owned(), 
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
            changes,
        };

        let msg = serde_json::to_vec(&msg).unwrap();
        producer.send(FutureRecord::to("far-memory-updates")
            .payload(&msg)
            .key(&random_key()),
            Duration::from_secs(10)
        ).await.unwrap();

        memory_changes_tracked += 1;
        println!("total memory changes tracked: {}", memory_changes_tracked);

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

fn compute_swapfile_hashes() -> Vec<String> {
    let mut hashes = Vec::new();

    let mut swapfile = File::open("/swapfile").unwrap();
    let mut buffer = vec![0; 1024 * 1024 * 1]; // read 1 megabyte at a time

    let pb = indicatif::ProgressBar::new(swapfile.metadata().unwrap().len());
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    loop {
        let read = swapfile.read(&mut buffer).unwrap();
        pb.inc(read as u64);
        if read == 0 {
            break;
        }

        let hash = FuzzyHash::new(&buffer[..read]).to_string();
        hashes.push(hash);
    }

    pb.finish();

    hashes
}

fn hashes_compare(a: &Vec<String>, b: &Vec<String>) -> Vec<u32> {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| FuzzyHash::compare(a, b).unwrap_or(0))
        .collect()
}

fn kafka_producer(endpoint: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", endpoint)
        .set("message.timeout.ms", "5000")
        .create()
        .unwrap()
}

fn random_key() -> Vec<u8> {
    let mut id = [0u8; 12];
    rand::thread_rng().fill(&mut id);
    id.to_vec()
}