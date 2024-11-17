use tracing::level_filters::LevelFilter;
use tracing_subscriber::{prelude::*, Registry};

pub fn start_stderr_logger() -> anyhow::Result<()> {
    let err = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_writer(std::io::stderr);

    let logger = Registry::default().with(LevelFilter::TRACE).with(err);

    tracing::subscriber::set_global_default(logger)?;
    Ok(())
}

