//! Solution for https://leetcode.com/problems/word-break-ii
//! 140. Word Break II

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

use std::cell::RefCell;
use std::rc::Rc;

type TriePtr = Box<Trie>;
const A: usize = 26;

struct Trie {
    terminal: bool,
    to: [Option<TriePtr>; A],
}

impl Trie {
    fn new() -> Self {
        Self {
            terminal: false,
            to: Default::default(),
        }
    }

    fn new_ptr() -> TriePtr {
        Box::new(Trie::new())
    }

    fn add(&mut self, w: &Vec<u8>) {
        let mut t = self;
        for &ch in w {
            let c = (ch - b'a') as usize;
            if t.to[c].is_none() {
                t.to[c] = Some(Trie::new_ptr());
            }
            t = t.to[c].as_mut().unwrap();
        }
        t.terminal = true;
    }
}

fn go(s: &Vec<u8>, i: usize, root: &Trie, cur_ans: &mut Vec<Vec<u8>>, ans: &mut Vec<Vec<u8>>) {
    // eprintln!("go, cur_ans len = {}", cur_ans.len());

    if i == s.len() {
        ans.push(cur_ans.join(&b' '));
        return;
    }

    let mut t = root;
    for j in i..s.len() {
        let c = (s[j] - b'a') as usize;
        if t.to[c].is_none() {
            break;
        }
        t = t.to[c].as_ref().unwrap();

        // eprintln!("found link by \"{}\"", s[j] as char);

        if t.terminal {
            cur_ans.push(s[i..j + 1].to_vec());
            go(s, j + 1, root, cur_ans, ans);
            cur_ans.pop();
        }
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let s = to_u8(&s);
        let word_dict = to_u8_vec(&word_dict);

        let mut root = Trie::new();
        for word in &word_dict {
            Trie::add(&mut root, &word);
        }

        let mut ans = Vec::<Vec<u8>>::new();
        let mut cur_ans = Vec::<Vec<u8>>::new();
        go(&s, 0, &root, &mut cur_ans, &mut ans);

        from_u8_vec(&ans)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("catsanddog", vec!["cat".into(),"cats".into(),"and".into(),"sand".into(),"dog".into()], vec!["cats and dog".into(),"cat sand dog".into()])]
    #[case("pineapplepenapple", vec!["apple".into(),"pen".into(),"applepen".into(),"pine".into(),"pineapple".into()], vec!["pine apple pen apple".into(),"pineapple pen apple".into(),"pine applepen apple".into()])]
    #[case("catsandog", vec!["cats".into(),"dog".into(),"sand".into(),"and".into(),"cat".into()], vec![])]
    fn case(#[case] s: String, #[case] word_dict: Vec<String>, #[case] expected: Vec<String>) {
        let actual = Solution::word_break(s, word_dict);
        assert_eq!(actual, expected);
    }
}
