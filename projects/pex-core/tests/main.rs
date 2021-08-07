use regex_automata::dfa::regex::Regex;
use ucd_trie::TrieSet;

use pex::ParseState;

#[test]
fn ready() {
    println!("it works!")
}

pub fn test_trie() {
    let trie = TrieSet::new(&["a", "b", "c"]);
    let text = "abc";
    let s = ParseState::new(text);
    let s = s.match_char_set(&trie, "data");
    println!("{:#?}", s)
}

#[test]
pub fn test_regex() {
    let re = Regex::new("[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
    let text = "2018-12-24 2016-10-08";
    let s = ParseState::new(text);
    let s = s.match_regex(&re, "data");
    println!("{:#?}", s)
}
