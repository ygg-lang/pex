#![doc = include_str ! ("../readme.md")]
#![doc(html_root_url = "https://docs.rs/pex-core/0.1.0")]
#![warn(missing_docs)]

pub mod generate;
mod regex_set;
mod unicode_set;
mod writer;

pub use unicode_set::UnicodeSet;
