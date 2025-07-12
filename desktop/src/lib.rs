pub mod config;
pub mod gui;
pub mod schema;

pub use gui::Component;
use tracing_subscriber::filter::EnvFilter;

pub fn setup_logger() {
    let filter = if cfg!(debug_assertions) {
        EnvFilter::new("noted=debug,info")
    } else {
        EnvFilter::new("noted=info,warn")
    }
    .add_directive("noted".parse().unwrap());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_test_writer()
        .init();
}
