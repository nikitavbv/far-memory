use {
    std::time::Instant,
    rand::Rng,
    indicatif::ProgressIterator,
    crate::benchmark::{InMemoryBackend, IOBackend},
};

pub mod benchmark;

fn main() {
    let mut backend = InMemoryBackend::new();
    let slot_data_size = 1024 * 1024;
    let total_slots = 4096;

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
        let started_at = Instant::now();

        let mut total_reads: i32 = 0;

        while (Instant::now() - started_at).as_secs() < 2 {
            let slot_to_read = rng.gen_range(0..total_slots);

            if benchmark_writes {
                // TODO: implement writes benchmark
            } else {
                let data = backend.read(slot_to_read);
        
                for i in 0..data.len() {
                    sum += data[i];
                }
            }
    
            total_reads += 1;
        }

        let ops_per_sec = (total_reads as f64) / (Instant::now() - started_at).as_secs_f64();
        results.push(ops_per_sec);
    }

    sum += backend.read(rng.gen_range(0..total_slots))[rng.gen_range(0..slot_data_size)];

    println!("done, sum: {}", sum);
    println!("mean: {}", statistical::mean(&results));
    println!("median: {}", statistical::median(&results));
    println!("standard deviation: {}", statistical::standard_deviation(&results, None));
    println!("variance: {}", statistical::variance(&results, None));
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