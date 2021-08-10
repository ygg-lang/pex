#![feature(try_trait_v2)]
#![feature(try_blocks)]
#![doc = include_str ! ("../readme.md")]
#![doc(html_root_url = "https://docs.rs/pex-core/0.1.0")]
#![warn(missing_docs)]

pub use crate::{
    results::{ParseResult, StopBecause},
    states::{advance::ParseAdvance, choice::ChoiceHelper, ParseState, Parsed},
};

mod results;
mod states;
mod third_party;
pub mod helpers;