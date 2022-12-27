use super::*;

/// A pattern that matches a number with a leading zero and a base mark,
/// such as `0x0123456789ABCDEF`, `0o01234567`, `0b01010101`.
///
///
/// # Examples
///
/// ```
/// # use pex::{ParseState, ZeroBytePattern};
/// let lower = ParseState::new("0x1234");
/// let upper = ParseState::new("0X1234");
/// let bytes = ZeroBytePattern::new(&[("0x", 16), ("0o", 8), ("0b", 2)]);
/// assert!(bytes.match_pattern(lower).is_success());
/// assert!(bytes.match_pattern(upper).is_failure());
/// let bytes = bytes.with_insensitive(true);
/// assert!(bytes.match_pattern(lower).is_success());
/// assert!(bytes.match_pattern(upper).is_success());
/// ```
#[derive(Copy, Clone, Debug)]
pub struct ZeroBytePattern {
    insensitive: bool,
    marks: &'static [(&'static str, u32)],
    message: &'static str,
}

impl ZeroBytePattern {
    /// Create a new `ZeroBytePattern` with a leading character and a list of marks.
    pub const fn new(marks: &'static [(&'static str, u32)]) -> Self {
        Self { insensitive: false, marks, message: "ZeroBytePattern" }
    }
    /// Create a new `ZeroBytePattern` with a leading character and a list of marks.
    pub const fn with_insensitive(self, insensitive: bool) -> Self {
        Self { insensitive, ..self }
    }
    /// Create a new `ZeroBytePattern` with a leading character and a list of marks.
    pub const fn with_message(self, message: &'static str) -> Self {
        Self { message, ..self }
    }
    /// Create a new `ZeroBytePattern` with a leading character and a list of marks.
    pub fn match_pattern<'i>(&self, state: ParseState<'i>) -> ParseResult<'i, (u32, &'i str)> {
        for (mark, base) in self.marks {
            match Self::parse_byte_base(state, *mark, *base, self.insensitive) {
                Pending(s, v) => return s.finish(v),
                _ => continue,
            }
        }
        StopBecause::missing_character_set(self.message, state.start_offset)?
    }
    /// Create a new `ZeroBytePattern` with a leading character and a list of marks.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pex::{ParseState, ZeroBytePattern};
    /// let lower = ParseState::new("0x1234");
    /// assert!(ZeroBytePattern::parse_byte_base(lower, "0X", 16, false).is_failure());
    /// assert!(ZeroBytePattern::parse_byte_base(lower, "0X", 16, true).is_success());
    /// ```
    pub fn parse_byte_base<'i>(
        state: ParseState<'i>,
        mark: &'static str,
        base: u32,
        insensitive: bool,
    ) -> ParseResult<'i, (u32, &'i str)> {
        let (state, _) = match insensitive {
            true => state.match_str_insensitive(mark)?,
            false => state.match_str(mark)?,
        };
        let mut offset = 0;
        for c in state.residual.chars() {
            match c {
                // it's a valid digit
                c if c.is_digit(base) => offset += 1,
                _ => break,
            }
        }
        // SAFETY: offset is always less than the length of the residual string
        let str = unsafe { state.residual.get_unchecked(..offset) };
        state.advance(offset).finish((base, str))
    }
}
