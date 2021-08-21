use crate::{ParseResult, ParseState, StopBecause};

/// Parse color tuple from string
///
/// | Input     | Output            |
/// |-----------|-------------------|
/// | `#A`        | (A, A, A, 255)    |
/// | `#AB`       | (AB, AB, AB, 255) |
/// | `#ABC`      | (A, B, C, 255)    |
/// | `#ABCD`     | (AA, BB, CC, DD)  |
/// | `=5`        | Error             |
/// | `#ABCDEF`   | (AB, CD, EF, 255) |
/// | `=7`        | Error             |
/// | `#ABCDEFGH` | (AB, CD, EF, GH)  |
/// | `>8`        | Error             |
pub fn hex_color<'a>(input: ParseState<'a>, start: &'static str) -> ParseResult<'a, (u8, u8, u8, u8)> {
    let state = if start.is_empty() { input } else { input.match_str(start, true)?.0 };
    let (state, hex) = state.match_str_if(|c| c.is_ascii_hexdigit(), "ASCII_HEX")?;
    // SAFETY: `hex` is guaranteed to be ASCII hex digits
    let color = match hex.as_bytes() {
        // gray
        [gray] => {
            let c = byte2_to_u8(gray, gray);
            (c, c, c, 255)
        }
        // gray
        [gray1, gray2] => {
            let c = byte2_to_u8(gray1, gray2);
            (c, c, c, 255)
        }
        // rgb
        [r, g, b] => {
            let r = byte2_to_u8(r, r);
            let g = byte2_to_u8(g, g);
            let b = byte2_to_u8(b, b);
            (r, g, b, 255)
        }
        // rgba
        [r, g, b, a] => {
            let r = byte2_to_u8(r, r);
            let g = byte2_to_u8(g, g);
            let b = byte2_to_u8(b, b);
            let a = byte2_to_u8(a, a);
            (r, g, b, a)
        }
        // rgb
        [r1, r2, g1, g2, b1, b2] => {
            let r = byte2_to_u8(r1, r2);
            let g = byte2_to_u8(g1, g2);
            let b = byte2_to_u8(b1, b2);
            (r, g, b, 255)
        }
        // rgba
        [r1, r2, g1, g2, b1, b2, a1, a2] => {
            let r = byte2_to_u8(r1, r2);
            let g = byte2_to_u8(g1, g2);
            let b = byte2_to_u8(b1, b2);
            let a = byte2_to_u8(a1, a2);
            (r, g, b, a)
        }
        buffer => StopBecause::custom_error("Color format wrong, except 1,2,3,4,6,8", state.start_offset + buffer.len())?,
    };
    state.advance(hex.len()).finish(color)
}

#[inline(always)]
fn byte2_to_u8(high: &u8, low: &u8) -> u8 {
    byte_to_u8(*high) << 4 | byte_to_u8(*low)
}

#[inline(always)]
fn byte_to_u8(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        b'A'..=b'F' => byte - b'A' + 10,
        _ => unreachable!(),
    }
}
