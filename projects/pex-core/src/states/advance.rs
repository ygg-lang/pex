use super::*;
use crate::ParseResult;

pub enum ParseAdvance {
    Offset(usize),
    Character(char),
    String(&'static str),
}

impl From<usize> for ParseAdvance {
    fn from(v: usize) -> Self {
        ParseAdvance::Offset(v)
    }
}

impl From<char> for ParseAdvance {
    fn from(c: char) -> Self {
        ParseAdvance::Character(c)
    }
}

impl From<&'static str> for ParseAdvance {
    fn from(s: &'static str) -> Self {
        ParseAdvance::String(s)
    }
}

impl<'i> ParseState<'i> {
    /// Advance the parser to a new state.
    #[inline]
    pub fn advance<T>(self, term: T) -> ParseState<'i>
    where
        T: Into<ParseAdvance>,
    {
        let offset = match term.into() {
            ParseAdvance::Offset(v) => v,
            ParseAdvance::Character(v) => v.len_utf8(),
            ParseAdvance::String(v) => v.len(),
        };
        ParseState {
            rest_text: &self.rest_text[offset..],
            start_offset: self.start_offset + offset,
            stop_reason: self.stop_reason,
        }
    }
    /// Advance the parser state and return the view of these string.
    #[inline]
    pub fn advance_view(self, offset: usize) -> ParseResult<'i, &'i str> {
        let view = &self.rest_text[0..offset];
        ParseState {
            rest_text: &self.rest_text[offset..],
            start_offset: self.start_offset + offset,
            stop_reason: self.stop_reason,
        }
        .finish(view)
    }
}
