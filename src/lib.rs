use std::io::stderr;

use time::macros::format_description;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    fmt,
};

pub fn setup_tracing() {
    let tracing_env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()
        .unwrap();

    let tracing_timer = fmt::time::UtcTime::new(format_description!(
        "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
    ));

    tracing_subscriber::fmt()
        .with_writer(stderr)
        .with_env_filter(tracing_env_filter)
        .with_timer(tracing_timer)
        .init();
}
