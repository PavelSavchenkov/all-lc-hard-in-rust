//! Solution for https://leetcode.com/problems/longest-common-suffix-queries
//! 3093. Longest Common Suffix Queries

type TriePtr = Option<Box<Trie>>;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct Val {
    len: usize,
    id: usize,
}

impl Val {
    fn new() -> Self {
        Self {
            len: usize::MAX,
            id: usize::MAX,
        }
    }
}

#[derive(Clone)]
struct Trie {
    to: Vec<TriePtr>,
    val: Val,
}

const A: usize = 26;

impl Trie {
    fn new() -> Self {
        Self {
            to: vec![None; A],
            val: Val::new(),
        }
    }

    fn add(&mut self, s: &Vec<u8>, id: usize) {
        let val = Val { len: s.len(), id };
        let mut t = self;
        t.val = t.val.min(val);
        for &ch in s {
            let c = (ch - b'a') as usize;
            t = t.to[c].get_or_insert(Box::new(Trie::new()));
            t.val = t.val.min(val);
        }
    }

    fn get_best_val(&self, s: &Vec<u8>) -> Val {
        let mut t = self;
        for &ch in s {
            let c = (ch - b'a') as usize;
            if let Some(son) = &t.to[c] {
                t = son;
            } else {
                return t.val;
            }
        }
        t.val
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut words_container = to_u8_vec(&words_container);
        let mut word_query = to_u8_vec(&words_query);

        for s in &mut words_container {
            s.reverse();
        }
        for s in &mut word_query {
            s.reverse();
        }

        let mut root = Trie::new();
        for i in 0..words_container.len() {
            root.add(&words_container[i], i);
        }

        let mut ans = Vec::with_capacity(word_query.len());
        for s in &word_query {
            let val = root.get_best_val(s);
            ans.push(val.id as i32);
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
    #[case(vec!["abcd".into(),"bcd".into(),"xbcd".into()], vec!["cd".into(),"bcd".into(),"xyz".into()], vec![1,1,1])]
    #[case(vec!["abcdefgh".into(),"poiuygh".into(),"ghghgh".into()], vec!["gh".into(),"acbfgh".into(),"acbfegh".into()], vec![2,0,2])]
    fn case(
        #[case] words_container: Vec<String>,
        #[case] words_query: Vec<String>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::string_indices(words_container, words_query);
        assert_eq!(actual, expected);
    }
}
