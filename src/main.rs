use std::fs::read_to_string;

use {
    std::{
        env, 
        fs::{File, OpenOptions}, 
        io::{Read, Write as IOWrite}, 
        fmt::Write, 
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
    fuzzyhash::FuzzyHash,
    indicatif::{ProgressStyle, ProgressState},
    rdkafka::{
        ClientConfig,
        Message,
        client::ClientContext,
        producer::{FutureProducer, FutureRecord},
        consumer::{StreamConsumer, ConsumerContext, Consumer},
    },
    serde::{Serialize, Deserialize},
    rand::Rng,
};

#[derive(Serialize, Deserialize)]
struct MemoryStateMessage {
    service: String,
    time: f64,
    changes: Vec<u32>,
}

#[derive(Serialize)]
struct MemoryPageState {
    service: String,
    time: f64,
    page: u32,
    change: u32,
}

pub struct StreamingContext;

impl ClientContext for StreamingContext {}

impl ConsumerContext for StreamingContext {}

#[derive(Deserialize)]
struct Config {
    kafka_endpoint: Option<String>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();

    storage_import_step(&config).await;

    Ok(())
}

async fn storage_import_step(config: &Config) {
    let consumer = kafka_consumer_for_topic(config.kafka_endpoint.as_ref().unwrap(), "far-memory-updates");
    let mut entries_saved = 0;

    loop {
        let msg = consumer.recv().await.unwrap();
        let payload = msg.payload().unwrap();

        let payload: MemoryStateMessage = serde_json::from_slice(&payload).unwrap();
        let output_data = {
            let mut output_data = Vec::new();

            {
                let mut csv_writer = csv::WriterBuilder::new()
                    .has_headers(false)
                    .from_writer(&mut output_data);

                for i in 0..payload.changes.len() {
                    csv_writer.serialize(&MemoryPageState {
                        service: payload.service.clone(),
                        time: payload.time,
                        page: i as u32,
                        change: *payload.changes.get(i).unwrap(),
                    }).unwrap(); 
                }
            }
            
            output_data
        };

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("./data.csv")
            .unwrap();

        file.write(&output_data).unwrap();

        entries_saved += 1;
        println!("entries saved: {}", entries_saved);
    }
}

async fn collect_memory_stats() {
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

fn load_config() -> Config {
    toml::from_str(&read_to_string("./config.toml").unwrap()).unwrap()
}

fn kafka_producer(endpoint: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", endpoint)
        .set("message.timeout.ms", "5000")
        .create()
        .unwrap()
}

fn kafka_consumer_for_topic(endpoint: &str, topic: &str) -> StreamConsumer<StreamingContext> {
    let consumer: StreamConsumer<StreamingContext> = ClientConfig::new()
        .set("group.id", "far-memory-storage-import-v2")
        .set("bootstrap.servers", endpoint)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "beginning")
        .create_with_context(StreamingContext)
        .unwrap();

    consumer.subscribe(&vec![topic]).unwrap();

    consumer
}

fn random_key() -> Vec<u8> {
    let mut id = [0u8; 12];
    rand::thread_rng().fill(&mut id);
    id.to_vec()
}