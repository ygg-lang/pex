use super::*;

pub fn dec_str<'i>(input: ParseState<'i>) -> ParseResult<&'i str> {
    let mut offset = 0;
    let mut first_dot = true;
    for char in input.rest_text.chars() {
        match char {
            '.' if first_dot => {
                first_dot = false;
                offset += 1;
            }
            '0'..='9' => {
                offset += 1;
            }
            _ => {
                break;
            }
        }
    }
    if offset == 0 {
        StopBecause::missing_string("DECIMAL_LITERAL", input.start_offset)?;
    }
    input.advance_view(offset)
}

pub fn dec_u128(state: ParseState) -> ParseResult<u128> {
    let (state, txt) = match_dec(state)?;
    match u128::from_str(txt) {
        Ok(o) => state.finish(o),
        Err(_) => StopBecause::missing_string("decimal u128", state.start_offset)?,
    }
}

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
            '0'..='9' => {
                offset += 1;
            }
            _ => break,
        }
    }
    if offset == 0 {
        StopBecause::missing_string("decimal digits", state.start_offset)?
    }
    state.advance_view(offset)
}

#[test]
fn test() {
    let out = dec_u128(ParseState::new(""));
    println!("{:?}", out);
}
