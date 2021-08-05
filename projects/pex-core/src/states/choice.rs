use crate::{Parsed, SResult, SResult::Stop};

use super::*;

#[derive(Debug, Clone)]
pub struct ChoiceHelper<'a, T> {
    state: YState<'a>,
    result: Option<Parsed<'a, T>>,
}

impl<'i> YState<'i> {
    /// Begin a choice
    #[inline]
    pub fn begin_choice<T>(self) -> ChoiceHelper<'i, T> {
        ChoiceHelper { state: self, result: None }
    }
}

impl<'a, T> ChoiceHelper<'a, T> {
    /// Create a new choice helper
    #[inline]
    pub fn new(state: YState<'a>) -> Self {
        Self { state, result: None }
    }

    /// Try to parse a value
    #[inline]
    pub fn maybe(mut self, parse_fn: impl FnOnce(YState<'a>) -> SResult<'a, T>) -> Self {
        if self.result.is_none() {
            match parse_fn(self.state.clone()) {
                Pending(s, v) => self.result = Some((s, v)),
                Stop(err) => self.state.set_error(err),
            }
        }
        self
    }
    /// End choice
    #[inline]
    pub fn end_choice(self) -> SResult<'a, T> {
        match self.result {
            Some(ok) => Pending(ok.0, ok.1),
            None => Stop(self.state.get_error()),
        }
    }
}
