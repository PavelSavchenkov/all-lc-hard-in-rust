//! Solution for https://leetcode.com/problems/count-prefix-and-suffix-pairs-ii
//! 3045. Count Prefix and Suffix Pairs II

fn calc_prefix_function(s: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let mut p = vec![0; n];
    for i in 1..n {
        let mut j = p[i - 1];
        while j > 0 && s[i] != s[j] {
            j = p[j - 1];
        }
        if s[i] == s[j] {
            p[i] = j + 1;
        }
    }
    p
}

const A: usize = 26;

type TriePtr = Option<Box<Trie>>;

#[derive(Clone)]
struct Trie {
    to: Vec<TriePtr>,
    size: usize,
}

impl Trie {
    fn new() -> Self {
        Self {
            to: vec![None; A],
            size: 0,
        }
    }

    fn add(&mut self, s: &Vec<u8>) {
        let p = calc_prefix_function(&s);
        let mut mark = vec![false; s.len()];
        let mut j = s.len();
        while j > 0 {
            mark[j - 1] = true;
            j = p[j - 1];
        }

        let mut t = self;
        t.size += 1;
        for i in 0..s.len() {
            let ch = s[i];
            let c = (ch - b'a') as usize;
            t = t.to[c].get_or_insert(Box::new(Trie::new()));
            if mark[i] {
                t.size += 1;
            }
        }
    }

    fn ask(&self, s: &Vec<u8>) -> usize {
        let mut t = self;
        for &ch in s {
            let c = (ch - b'a') as usize;
            if let Some(son) = &t.to[c] {
                t = &son;
            } else {
                return 0;
            }
        }
        t.size
    }
}

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
        let words = to_u8_vec(&words);

        let mut root = Trie::new();
        let mut ans = 0;
        for i in (0..words.len()).rev() {
            let cur = root.ask(&words[i]);
            ans += cur as i64;
            root.add(&words[i]);
        }
        ans
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
    #[case(vec!["a".into(),"aba".into(),"ababa".into(),"aa".into()], 4)]
    #[case(vec!["pa".into(),"papa".into(),"ma".into(),"mama".into()], 2)]
    #[case(vec!["abab".into(),"ab".into()], 0)]
    fn case(#[case] words: Vec<String>, #[case] expected: i64) {
        let actual = Solution::count_prefix_suffix_pairs(words);
        assert_eq!(actual, expected);
    }
}
