//! Solution for https://leetcode.com/problems/brace-expansion-ii
//! 1096. Brace Expansion II

#[derive(Default)]
struct Set {
    words: Vec<Vec<u8>>,
}

impl Set {
    fn new(ch: u8) -> Self {
        Self {
            words: vec![vec![ch; 1]; 1],
        }
    }

    fn to_vec_string(&self) -> Vec<String> {
        let mut ans = from_u8_vec(&self.words);
        ans.sort();
        ans.dedup();
        ans
    }

    fn concat(&mut self, other: Self) {
        if self.words.is_empty() {
            *self = other;
            return;
        }
        let mut new_words = Vec::new();
        for w0 in &self.words {
            for w1 in &other.words {
                let mut w = w0.clone();
                w.extend(w1.iter());
                new_words.push(w);
            }
        }
        self.words = new_words;
    }

    fn unite(&mut self, other: Self) {
        self.words.extend(other.words.into_iter());
    }
}

// single letter or {exp}
fn parse_atom(s: &Vec<u8>, i: usize) -> (Set, usize) {
    assert!(i < s.len());
    // eprintln!("parse atom: {}", from_u8(&s[i..].to_vec()));
    if s[i].is_ascii_lowercase() {
        return (Set::new(s[i]), i + 1);
    }
    assert!(s[i] == b'{');
    let (set, end) = parse_unite(s, i + 1);
    assert!(s[end] == b'}');
    (set, end + 1)
}

fn parse_unite(s: &Vec<u8>, mut i: usize) -> (Set, usize) {
    let mut set = Set::default();
    // eprintln!("parse unite: {}", from_u8(&s[i..].to_vec()));
    loop {
        let (sub, end) = parse_concat(s, i);
        set.unite(sub);
        if s[end] == b'}' {
            return (set, end);
        }
        assert!(s[end] == b',');
        i = end + 1;
    }
}

fn parse_concat(s: &Vec<u8>, mut i: usize) -> (Set, usize) {
    let mut set = Set::default();
    // eprintln!("parse concat: {}", from_u8(&s[i..].to_vec()));
    loop {
        let (sub, end) = parse_atom(s, i);
        set.concat(sub);
        if end == s.len() || s[end] == b',' || s[end] == b'}' {
            return (set, end);
        }
        i = end;
    }
}

impl Solution {
    pub fn brace_expansion_ii(s: String) -> Vec<String> {
        let s = to_u8(&s);

        let (set, i) = parse_concat(&s, 0);
        assert!(i == s.len());
        set.to_vec_string()
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("{a,b}{c,{d,e}}", vec!["ac".into(),"ad".into(),"ae".into(),"bc".into(),"bd".into(),"be".into()])]
    #[case("{{a,z},a{b,c},{ab,z}}", vec!["a".into(),"ab".into(),"ac".into(),"z".into()])]
    fn case(#[case] expression: String, #[case] expected: Vec<String>) {
        let actual = Solution::brace_expansion_ii(expression);
        assert_eq!(actual, expected);
    }
}
