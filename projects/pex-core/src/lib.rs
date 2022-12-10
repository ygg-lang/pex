#![feature(try_trait_v2, try_blocks)]
#![feature(const_mut_refs, const_for, const_try)]
#![feature(pattern)]
#![feature(unboxed_closures)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

pub use crate::{
    results::{ParseResult, StopBecause},
    states::{advance::ParseAdvance, choice::ChoiceHelper, ParseState, Parsed},
};

pub mod helpers;
mod patterns;
mod results;
mod states;
mod third_party;
pub use crate::patterns::{NamedPattern, StringView, SurroundPair, SurroundPairPattern};

#[cfg(feature = "ucd-trie")]
pub use ucd_trie::TrieSetSlice;
