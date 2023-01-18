use {
    std::{fs::File, io::Read},
    fuzzyhash::FuzzyHash,
};

fn main() {
    println!("computing hashes");
    let hashes = compute_swapfile_hashes();

    println!("computing more hashes");
    let more_hashes = compute_swapfile_hashes();

    println!("comparing");
    let compare_result = hashes_compare(&hashes, &more_hashes);
    println!("compare result: {:?}", compare_result);
}

fn compute_swapfile_hashes() -> Vec<String> {
    let mut hashes = Vec::new();

    let mut swapfile = File::open("/swapfile").unwrap();
    let mut buffer = vec![0; 1024 * 1024 * 8]; // read 8 megabytes at a time

    let pb = indicatif::ProgressBar::new(swapfile.metadata().unwrap().len());

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
        .map(|(a, b)| FuzzyHash::compare(a, b).unwrap())
        .collect()
}