use regex_automata::dfa::regex::Regex;

use pex::ParseState;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn test_re() {
    let re = Regex::new("[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
    let text = "2018-12-24 2016-10-08";
    let s = ParseState::new(text);
    let s = s.match_re(&re, "data");
    println!("{:?}", s)
}
