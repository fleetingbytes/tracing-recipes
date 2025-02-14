//! Create two layers: `stderr_log` and `file_log`.
//! File records all log levels, the stderr_log is filtered by the log level set in the env
//! variable TRACE_RECIPES_LOG_LEVEL.
//! Based on https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/targets/struct.Targets.html#examples

use smallvec::{smallvec, SmallVec};
use std::{fs::File, io::Error};
use tracing::{debug, error, info, trace, warn};
use tracing_appender::non_blocking;
use tracing_recipes::add;
use tracing_subscriber::{filter::EnvFilter, fmt, fmt::format::FmtSpan, prelude::*};

const NUMBER_OF_WORKERS_FOR_NON_BLOCKING_WRITERS: usize = 2;

// using SmallVec<[impl Drop; _]> is using _ and that is not stable yet; But soon!
// https://github.com/rust-lang/issues/85077
fn init_tracing() -> Result<SmallVec<[impl Drop; NUMBER_OF_WORKERS_FOR_NON_BLOCKING_WRITERS]>, Error>
{
    let (non_blocking_stderr, stderr_guard) = non_blocking(std::io::stderr());
    let stderr_log = fmt::layer()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_writer(non_blocking_stderr)
        .without_time()
        .pretty();

    let file = File::create("trace.log")?;
    let (non_blocking_file, file_guard) = non_blocking(file);
    let file_log = fmt::layer()
        .with_writer(non_blocking_file)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);

    tracing_subscriber::registry()
        .with(stderr_log.with_filter(EnvFilter::from_env("TRACE_RECIPES_LOG_LEVEL")))
        .with(file_log)
        .init();

    Ok(smallvec![stderr_guard, file_guard])
}

fn main() -> Result<(), Error> {
    let _guards = init_tracing()?;
    println!("{}", add(2, 4));
    trace!("demo trace message");
    debug!("demo debug message");
    info!("demo info message");
    warn!("demo warning message");
    error!("demo error message");
    Ok(())
}
