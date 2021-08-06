#![feature(try_trait_v2)]

pub use crate::results::{SResult, StopBecause};
pub use crate::states::{Parsed, YState};

mod errors;
mod results;
mod states;

