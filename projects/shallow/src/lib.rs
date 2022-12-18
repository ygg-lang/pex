#![no_std]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/91894079")]

extern crate alloc;

mod char_level;
mod string;
mod word_level;

pub use self::{
    char_level::{CharacterShallow, CounterMode, ShallowMode},
    string::ShallowString,
    word_level::WordShallow,
};
