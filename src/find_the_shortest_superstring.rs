//! Solution for https://leetcode.com/problems/find-the-shortest-superstring
//! 943. Find the Shortest Superstring

use std::collections::VecDeque;

const A: usize = 26;

#[derive(Debug, Clone)]
struct Node {
    to: Vec<Option<usize>>,
    go: Vec<usize>,
    who_ends: Option<usize>,
    link: Option<usize>,
}

impl Node {
    fn new() -> Self {
        Self {
            to: vec![None; A],
            go: Vec::new(),
            who_ends: None,
            link: None,
        }
    }
}

impl Solution {
    pub fn shortest_superstring(words: Vec<String>) -> String {
        let words = to_u8_vec(&words);
        let n = words.len();

        let mut nodes = Vec::new();
        nodes.push(Node::new());
        for i in 0..n {
            let mut v = 0;
            for ch in &words[i] {
                let c = (ch - b'a') as usize;
                if nodes[v].to[c].is_none() {
                    nodes[v].to[c] = Some(nodes.len());
                    nodes.push(Node::new());
                }
                v = nodes[v].to[c].unwrap();
            }
            nodes[v].who_ends = Some(i);
        }

        // suffix links and go
        {
            let mut q = VecDeque::new();
            q.push_back(0);
            while !q.is_empty() {
                let v = q.pop_front().unwrap();
                // link is ready, go is not
                nodes[v].go = vec![0; A];
                for c in 0..A {
                    if let Some(to) = nodes[v].to[c] {
                        nodes[v].go[c] = to;
                    } else if v != 0 {
                        let u = nodes[v].link.unwrap();
                        nodes[v].go[c] = nodes[u].go[c];
                    }
                }

                for c in 0..A {
                    if let Some(to) = nodes[v].to[c] {
                        if v == 0 {
                            nodes[to].link = Some(0);
                        } else {
                            let u = nodes[v].link.unwrap();
                            nodes[to].link = Some(nodes[u].go[c]);
                        }
                        q.push_back(to);
                    }
                }
            }
        }

        let mut dp = vec![vec![usize::MAX; nodes.len()]; 1 << n];
        let mut prev = vec![vec![(0, 0, 0); nodes.len()]; 1 << n];
        dp[0][0] = 0;
        let mut q = VecDeque::new();
        q.push_front((0, 0));
        while !q.is_empty() {
            let (mask, v) = q.pop_front().unwrap();
            let cur_dp = dp[mask][v];
            for c in 0..A {
                let nv = nodes[v].go[c];
                let mut nmask = mask;
                if let Some(i) = nodes[nv].who_ends {
                    nmask |= 1 << i;
                }
                if dp[nmask][nv] > cur_dp + 1 {
                    dp[nmask][nv] = cur_dp + 1;
                    prev[nmask][nv] = (mask, v, c);
                    q.push_back((nmask, nv));
                }
            }
        }

        let mut ans = usize::MAX;
        let full_mask = (1 << n) - 1;
        let mut v0 = 0;
        for v in 0..nodes.len() {
            let cur_dp = dp[full_mask][v];
            if cur_dp < ans {
                ans = cur_dp;
                v0 = v;
            }
        }
        assert!(ans < usize::MAX);

        let mut mask = full_mask;
        let mut v = v0;
        let mut str = Vec::new();
        for it in 0..ans {
            let (nmask, nv, c) = prev[mask][v];
            str.push(b'a' + c as u8);
            mask = nmask;
            v = nv;
        }
        str.reverse();
        from_u8(&str)
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
    #[case(vec!["alex".into(),"loves".into(),"leetcode".into()], "alexlovesleetcode")]
    #[case(vec!["catg".into(),"ctaagt".into(),"gcta".into(),"ttca".into(),"atgcatc".into()], "gctaagttcatgcatc")]
    fn case(#[case] words: Vec<String>, #[case] expected: String) {
        let actual = Solution::shortest_superstring(words);
        assert_eq!(actual, expected);
    }
}
