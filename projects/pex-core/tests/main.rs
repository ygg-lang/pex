use pex::{helpers::decimal_string, ParseResult, ParseState};
use std::str::FromStr;

#[test]
fn ready() {
    println!("it works!")
}

// decimal = integer (. integer)?
fn parse_decimal(state: ParseState) -> ParseResult<f64> {
    let (state, a) = decimal_string(state)?;
    let a = f64::from_str(a)?;
    state.finish(a)
}
