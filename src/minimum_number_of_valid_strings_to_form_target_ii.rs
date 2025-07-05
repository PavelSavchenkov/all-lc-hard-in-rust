//! Solution for https://leetcode.com/problems/minimum-number-of-valid-strings-to-form-target-ii
//! 3292. Minimum Number of Valid Strings to Form Target II

use std::collections::VecDeque;

const A: usize = 26;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    to: Vec<Option<usize>>,
    go: Vec<usize>,
    link: Option<usize>,
    terminal: bool,
    len: usize,
}

impl Node {
    fn new() -> Self {
        Self {
            to: vec![None; A],
            go: Vec::new(),
            link: None,
            terminal: false,
            len: 0,
        }
    }
}

struct Aho {
    nodes: Vec<Node>,
    root: usize,
}

impl Aho {
    fn new(words: Vec<String>) -> Self {
        let words = to_u8_vec(&words);
        let mut nodes = Vec::new();
        nodes.push(Node::new());
        for word in &words {
            let mut v = 0;
            for ch in word {
                let c = (ch - b'a') as usize;
                if nodes[v].to[c].is_none() {
                    let id = nodes.len();
                    nodes.push(Node::new());
                    nodes[id].len = nodes[v].len + 1;
                    nodes[v].to[c] = Some(id);
                }
                v = nodes[v].to[c].unwrap();
            }
            nodes[v].terminal = true;
        }

        // build suffix links and go
        let mut q = VecDeque::new();
        q.push_back(0);
        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            // link is ready, go is not
            // 1. build nodes[v].go
            nodes[v].go = vec![0; A];
            for c in 0..A {
                if let Some(to) = nodes[v].to[c] {
                    nodes[v].go[c] = to;
                } else {
                    let link = nodes[v].link;
                    if let Some(from) = link {
                        nodes[v].go[c] = nodes[from].go[c];
                    }
                }
            }
            // 2. build links of to such that v --> to
            for c in 0..A {
                if let Some(to) = nodes[v].to[c] {
                    let my_link = nodes[v].link;
                    if let Some(from) = my_link {
                        nodes[to].link = Some(nodes[from].go[c]);
                    } else {
                        nodes[to].link = Some(0);
                    }
                    q.push_back(to);
                }
            }
        }

        q.clear();
        q.push_back(0);
        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            if v != 0 {
                let from = nodes[v].link.unwrap();
                if nodes[from].terminal {
                    nodes[v].terminal = true;
                }
            }
            for c in 0..A {
                if let Some(to) = nodes[v].to[c] {
                    q.push_front(to);
                }
            }
        }

        Self { nodes, root: 0 }
    }

    fn process(&self, s: &Vec<u8>) -> usize {
        let n = s.len();
        let mut dp = vec![usize::MAX; n + 1];
        dp[0] = 0;
        let mut v = self.root;
        for i in 0..n {
            let c = (s[i] - b'a') as usize;
            v = self.nodes[v].go[c];
            let suff = self.nodes[v].len;
            if suff == 0 {
                return usize::MAX;
            }
            let j = i + 1 - suff;
            if dp[j] == usize::MAX {
                return usize::MAX;
            }
            dp[i + 1] = dp[j] + 1;
        }
        dp[n]
    }
}

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let s = to_u8(&target);

        let aho = Aho::new(words);
        let ans = aho.process(&s);
        if ans == usize::MAX {
            return -1;
        }
        ans as i32
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
    #[case(vec!["abc".into(),"aaaaa".into(),"bcdef".into()], "aabcdabc", 3)]
    #[case(vec!["abababab".into(),"ab".into()], "ababaababa", 2)]
    #[case(vec!["abcdef".into()], "xyz", -1)]
    fn case(#[case] words: Vec<String>, #[case] target: String, #[case] expected: i32) {
        let actual = Solution::min_valid_strings(words, target);
        assert_eq!(actual, expected);
    }
}
