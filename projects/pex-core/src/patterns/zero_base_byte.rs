use crate::{ParseResult, ParseResult::Pending, ParseState, StopBecause};

pub struct ZeroBytePattern {
    leading: char,
    marks: &'static [(char, u32)],
    message: &'static str,
}

impl ZeroBytePattern {
    pub fn match_pattern(self, state: ParseState) -> ParseResult<(char, &str)> {
        let (state, _) = state.match_char(self.leading)?;
        assert!(self.marks.len() > 0, "ZeroBytePattern must have at least one mark");
        for (mark, base) in self.marks {
            match parse_byte_base(state, *mark, *base) {
                Pending(s, v) => return s.finish(v),
                _ => continue,
            }
        }
        StopBecause::missing_character_set(self.message, state.start_offset)?
    }
}

fn parse_byte_base<'i>(state: ParseState<'i>, mark: char, base: u32) -> ParseResult<(char, &'i str)> {
    let (state, _) = state.match_char(mark)?;
    let (state, str) = state.match_str_if(|c| c.is_digit(base), "BASE")?;
    state.finish((mark, str))
}
