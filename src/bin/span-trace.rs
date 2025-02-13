use std::io::Error;
use tracing_recipes::add;
use tracing_subscriber::{fmt, fmt::format::FmtSpan, prelude::*};

fn init_tracing() -> () {
    tracing_subscriber::registry()
        .with(fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .init();
}

fn main() -> Result<(), Error> {
    init_tracing();
    println!("{}", add(2, 4));
    Ok(())
}
