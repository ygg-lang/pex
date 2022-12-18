use regex_automata::dfa::{dense, regex::Regex};

pub struct RegexDenseSet {
    name: String,
    dfa_namepath: String,
    regex: Regex,
    max_width: usize,
    left_indent: usize,
}

impl RegexDenseSet {
    pub fn export_rust_code(&self) -> Result<String, std::fmt::Error> {
        let name = self.name.as_str();
        let indent = self.left_indent;

        let mut code = format!("#[rustfmt::skip]\nconst {name}: TrieSetSlice<'static> = TrieSetSlice");
        code.push_str(" {\n");
        todo!()
    }
}

#[test]
pub fn test_regex() {
    let re = Regex::new("[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
    let (fwd_bytes, fwd_pad) = re.forward().to_bytes_little_endian();
    let (rev_bytes, rev_pad) = re.reverse().to_bytes_little_endian();
    #[rustfmt::skip] #[cfg(target_endian = "little")]
        let re2 = unsafe {
        println!("{}: {:?}", fwd_pad, &fwd_bytes[fwd_pad..]);
        println!("{}: {:?}", rev_pad, &rev_bytes[rev_pad..]);
        let fwd: dense::DFA<&[u32]> = dense::DFA::from_bytes_unchecked(&fwd_bytes[fwd_pad..]).unwrap().0;
        let rev: dense::DFA<&[u32]> = dense::DFA::from_bytes_unchecked(&rev_bytes[rev_pad..]).unwrap().0;
        Regex::builder().build_from_dfas(fwd, rev)
    };
    let (fwd_bytes, fwd_pad) = re.forward().to_bytes_big_endian();
    let (rev_bytes, rev_pad) = re.reverse().to_bytes_big_endian();
    #[cfg(target_endian = "big")]
    let re3 = unsafe {
        println!("{}: {:?}", fwd_pad, &fwd_bytes[fwd_pad..]);
        println!("{}: {:?}", rev_pad, &rev_bytes[rev_pad..]);
        let fwd: dense::DFA<&[u32]> = dense::DFA::from_bytes_unchecked(&fwd_bytes[fwd_pad..]).unwrap().0;
        let rev: dense::DFA<&[u32]> = dense::DFA::from_bytes_unchecked(&rev_bytes[rev_pad..]).unwrap().0;
        Regex::builder().build_from_dfas(fwd, rev)
    };
}
