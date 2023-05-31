use {
    std::time::Instant,
    rand::Rng,
    indicatif::ProgressIterator,
    crate::benchmark::{InMemoryBackend, IOBackend, OnDiskBackend, RemoteBackend},
};

pub mod benchmark;

fn main() {
    let mut backend = InMemoryBackend::new();
    //let mut backend = OnDiskBackend::new("data/benchmark".to_owned());
    //let mut backend = RemoteBackend::new("redis://host:6379");

    let slot_data_size = 2 * 1024 * 1024;
    let total_slots = 1024 * 10;

    println!("filling backend with data");
    fill_with_data(&mut backend, slot_data_size, total_slots);
    println!("done filling backend with data");

    println!("benchmarking read speed");
    run_benchmark(&mut backend, total_slots, slot_data_size, false);

    println!("benchmarking write speed");
    run_benchmark(&mut backend, total_slots, slot_data_size, true);
}

fn run_benchmark<T: IOBackend>(backend: &mut T, total_slots: usize, slot_data_size: usize, benchmark_writes: bool) {
    let mut rng = rand::thread_rng();

    let mut sum = 0;

    let mut results = Vec::new();
    for _ in (0..200).progress() {
        let data_to_write = backend.read(rng.gen_range(0..total_slots)).to_vec();

        let started_at = Instant::now();

        let mut total_reads: i32 = 0;

        while (Instant::now() - started_at).as_secs() < 2 {
            let slot = rng.gen_range(0..total_slots);

            if benchmark_writes {
                backend.write(slot, data_to_write.clone());
            } else {
                let data = backend.read(slot);
        
                for i in 0..data.len() {
                    sum += data[i];
                }
            }
    
            total_reads += 1;
        }

        let time_per_ops = (Instant::now() - started_at).as_nanos() as f64 / (total_reads as f64);
        results.push(time_per_ops);
    }

    sum += backend.read(rng.gen_range(0..total_slots))[rng.gen_range(0..slot_data_size)];

    println!("done, sum: {}", sum);
    println!("mean: {}", statistical::mean(&results).round());
    println!("median: {}", statistical::median(&results).round());
    println!("standard deviation: {}", statistical::standard_deviation(&results, None).round());
    println!("variance: {}", statistical::variance(&results, None).round());
}

fn fill_with_data<T: IOBackend>(backend: &mut T, slot_data_size: usize, total_slots: usize) {
    for slot in (0..total_slots).progress() {
        backend.write(slot, generate_data_for_slot(slot_data_size));
    }
}

fn generate_data_for_slot(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let mut buffer = Vec::new();
    while buffer.len() < size {
        buffer.push(rng.gen());
    }

    buffer
}