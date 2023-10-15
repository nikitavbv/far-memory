use {
    tracing_chrome::{ChromeLayerBuilder, FlushGuard},
    tracing_subscriber::prelude::*,
};

pub mod allocator;
pub mod performance;

pub fn init_logging() {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .init();
}

pub fn init_tracing() -> FlushGuard {
    let (chrome_layer, guard) = ChromeLayerBuilder::new().file("./trace.json").include_args(true).build();
    tracing_subscriber::registry().with(chrome_layer).init();
    guard
}
