use super::*;

/// Match decimal string for later use
pub fn dec_str<'i>(input: ParseState<'i>) -> ParseResult<&'i str> {
    let mut offset = 0;
    let mut first_dot = true;
    for char in input.rest_text.chars() {
        match char {
            '.' if first_dot => {
                first_dot = false;
                offset += 1;
            }
            '0'..='9' => offset += 1,
            _ => break,
        }
    }
    if offset == 0 {
        StopBecause::missing_string("DECIMAL_LITERAL", input.start_offset)?;
    }
    input.advance_view(offset)
}

/// Match and parse a decimal string into a [f64]
pub fn dec_f64(state: ParseState) -> ParseResult<f64> {
    let (state, txt) = dec_str(state)?;
    match f64::from_str(txt) {
        Ok(o) => state.finish(o),
        Err(_) => StopBecause::missing_string("decimal f64", state.start_offset)?,
    }
}

/// Match and parse a decimal integer into a [i64]
pub fn dec_u128(state: ParseState) -> ParseResult<u128> {
    let (state, txt) = match_dec(state)?;
    match u128::from_str(txt) {
        Ok(o) => state.finish(o),
        Err(_) => StopBecause::missing_string("decimal u128", state.start_offset)?,
    }
}
/// Match and parse a decimal integer into a [usize]
pub fn dec_usize(state: ParseState) -> ParseResult<usize> {
    let (state, txt) = match_dec(state)?;
    match usize::from_str(txt) {
        Ok(o) => state.finish(o),
        Err(_) => StopBecause::missing_string("decimal usize", state.start_offset)?,
    }
}

fn match_dec(state: ParseState) -> ParseResult<&str> {
    let mut offset = 0;
    for c in state.rest_text.chars() {
        match c {
            '0'..='9' => offset += 1,
            _ => break,
        }
    }
    if offset == 0 {
        StopBecause::missing_character_set("decimal digits", state.start_offset)?
    }
    state.advance_view(offset)
}
