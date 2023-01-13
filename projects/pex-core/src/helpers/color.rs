use crate::{utils::hex2_to_u8, ParseResult, ParseState, StopBecause};

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
#[derive(Clone, Copy, Debug)]
pub struct HexColor {
    head: &'static str,
}

impl Default for HexColor {
    fn default() -> Self {
        Self { head: "#" }
    }
}

impl HexColor {
    /// Create a new hex color parser
    pub fn new(head: &'static str) -> Self {
        Self { head }
    }
}

impl<'i> FnOnce<(ParseState<'i>,)> for HexColor {
    type Output = ParseResult<'i, (u8, u8, u8, u8)>;

    extern "rust-call" fn call_once(self, (input,): (ParseState<'i>,)) -> Self::Output {
        let state = if self.head.is_empty() { input } else { input.match_str(self.head)?.0 };
        let (state, hex) = state.match_str_if(|c| c.is_ascii_hexdigit(), "ASCII_HEX")?;
        // SAFETY: `hex` is guaranteed to be ASCII hex digits
        let color = match hex.as_bytes() {
            // gray
            [gray] => {
                let c = hex2_to_u8(*gray, *gray).unwrap();
                (c, c, c, 255)
            }
            // gray
            [gray1, gray2] => {
                let c = hex2_to_u8(*gray1, *gray2).unwrap();
                (c, c, c, 255)
            }
            // rgb
            [r, g, b] => {
                let r = hex2_to_u8(*r, *r).unwrap();
                let g = hex2_to_u8(*g, *g).unwrap();
                let b = hex2_to_u8(*b, *b).unwrap();
                (r, g, b, 255)
            }
            // rgba
            [r, g, b, a] => {
                let r = hex2_to_u8(*r, *r).unwrap();
                let g = hex2_to_u8(*g, *g).unwrap();
                let b = hex2_to_u8(*b, *b).unwrap();
                let a = hex2_to_u8(*a, *a).unwrap();
                (r, g, b, a)
            }
            // rgb
            [r1, r2, g1, g2, b1, b2] => {
                let r = hex2_to_u8(*r1, *r2).unwrap();
                let g = hex2_to_u8(*g1, *g2).unwrap();
                let b = hex2_to_u8(*b1, *b2).unwrap();
                (r, g, b, 255)
            }
            // rgba
            [r1, r2, g1, g2, b1, b2, a1, a2] => {
                let r = hex2_to_u8(*r1, *r2).unwrap();
                let g = hex2_to_u8(*g1, *g2).unwrap();
                let b = hex2_to_u8(*b1, *b2).unwrap();
                let a = hex2_to_u8(*a1, *a2).unwrap();
                (r, g, b, a)
            }
            buffer => StopBecause::custom_error(
                "Color format wrong, except 1,2,3,4,6,8",
                state.start_offset + buffer.len(),
                state.start_offset + buffer.len() + 1,
            )?,
        };
        state.advance(hex.len()).finish(color)
    }
}
