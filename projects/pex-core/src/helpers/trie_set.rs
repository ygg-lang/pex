use crate::{ParseResult, ParseState};
use core::fmt::{Debug, Formatter};

/// Compact character set trie tree
#[derive(Clone, Copy)]
pub struct CharactersTrie {
    /// name of the trie set
    pub set_name: &'static str,
    /// first tree, one level
    pub tree1_level1: &'static [u64],
    /// second tree, first level
    pub tree2_level1: &'static [u8],
    /// second tree, second level
    pub tree2_level2: &'static [u64],
    /// third tree, first level
    pub tree3_level1: &'static [u8],
    /// third tree, second level
    pub tree3_level2: &'static [u8],
    /// third tree, third level
    pub tree3_level3: &'static [u64],
}

impl Debug for CharactersTrie {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "CharacterSet(<{}>)", self.set_name)
    }
}

impl<'i> FnOnce<(ParseState<'i>,)> for CharactersTrie {
    type Output = ParseResult<'i, char>;
    #[inline]
    extern "rust-call" fn call_once(self, (input,): (ParseState<'i>,)) -> Self::Output {
        input.match_char_if(|c| self.contains(c), self.set_name)
    }
}

impl<'i> FnMut<(ParseState<'i>,)> for CharactersTrie {
    #[inline]
    extern "rust-call" fn call_mut(&mut self, (input,): (ParseState<'i>,)) -> Self::Output {
        FnOnce::call_once(*self, (input,))
    }
}

impl<'i> Fn<(ParseState<'i>,)> for CharactersTrie {
    #[inline]
    extern "rust-call" fn call(&self, (input,): (ParseState<'i>,)) -> Self::Output {
        FnOnce::call_once(*self, (input,))
    }
}

const CHUNK_SIZE: usize = 64;

impl CharactersTrie {
    /// Returns true if and only if the given Unicode scalar value is in this
    /// set.
    pub fn contains(&self, c: char) -> bool {
        self.contains_inner(c as usize)
    }

    /// Returns true if and only if the given codepoint is in this set.
    ///
    /// If the given value exceeds the codepoint range (i.e., it's greater
    /// than `0x10FFFF`), then this returns false.
    pub fn contains_u32(&self, cp: u32) -> bool {
        if cp > 0x10FFFF {
            return false;
        }
        self.contains_inner(cp as usize)
    }

    #[inline(always)]
    fn contains_inner(&self, cp: usize) -> bool {
        if cp < 0x800 {
            self.chunk_contains(cp, self.tree1_level1[cp >> 6])
        }
        else if cp < 0x10000 {
            let leaf = match self.tree2_level1.get((cp >> 6) - 0x20) {
                None => return false,
                Some(&leaf) => leaf,
            };
            self.chunk_contains(cp, self.tree2_level2[leaf as usize])
        }
        else {
            let child = match self.tree3_level1.get((cp >> 12) - 0x10) {
                None => return false,
                Some(&child) => child,
            };
            let i = ((child as usize) * CHUNK_SIZE) + ((cp >> 6) & 0b111111);
            let leaf = self.tree3_level2[i];
            self.chunk_contains(cp, self.tree3_level3[leaf as usize])
        }
    }

    #[inline(always)]
    fn chunk_contains(&self, cp: usize, chunk: u64) -> bool {
        ((chunk >> (cp & 0b111111)) & 1) == 1
    }
}
