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
pub fn rgba<'a>(state: ParseState<'a>, start: &'static str) -> ParseResult<'a, (u8, u8, u8, u8)> {
    let (state, _) = state.match_str(start, true)?;
    let mut buffer = Vec::with_capacity(8);
    for c in state.rest_text.chars() {
        match c {
            '0'..='9' | 'a'..='f' | 'A'..='F' => {
                buffer.push(c);
            }
            _ if buffer.len() > 8 => {
                StopBecause::custom_error("Color too long, except 1,2,3,4,6,8", state.start_offset + buffer.len())?;
            }
            _ => break,
        }
    }
    let color = match buffer.as_slice() {
        // gray
        [g] => {
            let c = build_u8(g);
            (c, c, c, 255)
        }
        // gray
        [g1, g2] => {
            let c = build_u8x2(g1, g2);
            (c, c, c, 255)
        }
        // rgb
        [r, g, b] => {
            let r = build_u8(r);
            let g = build_u8(g);
            let b = build_u8(b);
            (r, g, b, 255)
        }
        // rgba
        [r, g, b, a] => {
            let r = build_u8(r);
            let g = build_u8(g);
            let b = build_u8(b);
            let a = build_u8(a);
            (r, g, b, a)
        }
        // rgb
        [r1, r2, g1, g2, b1, b2] => {
            let r = build_u8x2(r1, r2);
            let g = build_u8x2(g1, g2);
            let b = build_u8x2(b1, b2);
            (r, g, b, 255)
        }
        // rgba
        [r1, r2, g1, g2, b1, b2, a1, a2] => {
            let r = build_u8x2(r1, r2);
            let g = build_u8x2(g1, g2);
            let b = build_u8x2(b1, b2);
            let a = build_u8x2(a1, a2);
            (r, g, b, a)
        }
        _ => StopBecause::custom_error("Color format wrong, except 1,2,3,4,6,8", state.start_offset + buffer.len())?,
    };
    state.advance(buffer.len()).finish(color)
}

#[inline(always)]
fn build_u8(c: &char) -> u8 {
    let n = *c as u8;
    match n {
        b'0'..=b'9' => n - b'0',
        b'a'..=b'f' => n - b'a' + 10,
        b'A'..=b'F' => n - b'A' + 10,
        _ => unreachable!(),
    }
}

#[inline(always)]
fn build_u8x2(c1: &char, c2: &char) -> u8 {
    let c1 = build_u8(c1);
    let c2 = build_u8(c2);
    c1 * 16 + c2
}
