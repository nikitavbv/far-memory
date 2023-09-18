pub fn init_logging() {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .init();
}

pub fn mm_to_twentieth_of_a_point(mm: f32) -> i32 {
    (mm * 56.6929133858).round() as i32
}
