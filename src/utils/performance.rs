use std::{sync::atomic::{AtomicU64, Ordering}, time::{Instant, Duration}, thread, fs, fmt::Display};

pub static COUNTER_SWAP_IN: Counter = Counter::new();
pub static COUNTER_SWAP_OUT: Counter = Counter::new();

pub struct Counter {
    value: AtomicU64,
}

pub struct Measurement {
    started_at: Instant,
}

impl Counter {
    pub const fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
        }
    }

    pub fn add(&self, measurement: Measurement) {
        self.value.fetch_add((Instant::now() - measurement.started_at).as_micros() as u64, Ordering::Relaxed);
    }

    pub fn measure() -> Measurement {
        Measurement {
            started_at: Instant::now(),
        }
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{}ms", self.value.load(Ordering::Relaxed) / 1000).fmt(f)
    }
}

pub fn run_performance_reporting_thread() {
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(30));

            write_performance_report();
        }
    });
}

pub fn write_performance_report() {
    let report = format!(
        "swap in: {}\nswap out: {}",
        COUNTER_SWAP_IN,
        COUNTER_SWAP_OUT
    );

    fs::write("./performance.txt", report).unwrap();
}