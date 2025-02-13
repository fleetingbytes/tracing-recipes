use std::{fs::File, io::Error, sync::Arc};
use tracing::{debug, error, info, trace, warn};
use tracing_recipes::add;
use tracing_subscriber::{filter::EnvFilter, fmt, fmt::format::FmtSpan, prelude::*};

fn init_tracing() -> Result<(), Error> {
    let stdout_log = fmt::layer()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .pretty();

    let file = File::create("trace.log")?;
    let file_log = fmt::layer()
        .with_writer(Arc::new(file))
        .with_span_events(FmtSpan::FULL);

    tracing_subscriber::registry()
        .with(stdout_log.with_filter(EnvFilter::from_env("TRACE_RECIPES_LOG_LEVEL")))
        .with(file_log)
        .init();

    Ok(())
}

fn main() -> Result<(), Error> {
    init_tracing()?;
    println!("{}", add(2, 4));
    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warning message");
    error!("error message");
    Ok(())
}
