//
use bpaf::*;
fn try_to_get_version() -> Result<usize, &'static str> {
    Ok(42)
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
//
#[allow(dead_code)]
pub struct Options {
    #[bpaf(argument("VERS"), fallback_with(try_to_get_version))]
    version: usize,
}
