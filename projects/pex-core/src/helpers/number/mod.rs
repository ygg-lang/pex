use super::*;

/// Match decimal string for later use
pub fn decimal_string<'i>(input: ParseState<'i>) -> ParseResult<&'i str> {
    let mut offset = 0;
    let mut first_dot = true;
    for char in input.residual.chars() {
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
