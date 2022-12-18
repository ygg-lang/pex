#![no_std]
#![feature(try_trait_v2, try_blocks)]
#![feature(const_mut_refs, const_for, const_try)]
#![feature(error_in_core)]
#![feature(pattern)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/91894079")]

extern crate alloc;

pub use crate::{
    patterns::{
        surround_pair::{SurroundPair, SurroundPairPattern},
        NamedPattern, StringView,
    },
    results::{ParseResult, StopBecause},
    states::{advance::ParseAdvance, choice::ChoiceHelper, ParseState, Parsed},
};

pub mod helpers;
mod patterns;
mod results;
mod states;
mod third_party;

#[cfg(feature = "ucd-trie")]
pub use ucd_trie::TrieSetSlice;
