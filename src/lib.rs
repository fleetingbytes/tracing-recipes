use tracing::{instrument, Span};

#[instrument(fields(result))]
pub fn add(left: u64, right: u64) -> u64 {
    let result = left + right;
    Span::current().record("result", &result);
    result
}
