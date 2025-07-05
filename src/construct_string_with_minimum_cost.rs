//! Solution for https://leetcode.com/problems/construct-string-with-minimum-cost
//! 3213. Construct String with Minimum Cost

use std::collections::VecDeque;

const A: usize = 26;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    to: Vec<Option<usize>>,
    go: Vec<usize>,
    link: Option<usize>,
    link_terminal: Option<usize>,
    cost: i32,
    len: usize,
}

impl Node {
    fn new() -> Self {
        Self {
            to: vec![None; A],
            go: Vec::new(),
            link: None,
            link_terminal: None,
            cost: i32::MAX,
            len: 0,
        }
    }
}

struct Aho {
    nodes: Vec<Node>,
    root: usize,
}

impl Aho {
    fn new(words: Vec<String>, costs: Vec<i32>) -> Self {
        assert!(words.len() == costs.len());
        let words = to_u8_vec(&words);
        let mut nodes = Vec::new();
        nodes.push(Node::new());
        for i in 0..words.len() {
            let mut v = 0;
            for ch in &words[i] {
                let c = (ch - b'a') as usize;
                if nodes[v].to[c].is_none() {
                    let id = nodes.len();
                    nodes[v].to[c] = Some(id);
                    nodes.push(Node::new());
                    nodes[id].len = nodes[v].len + 1;
                }
                v = nodes[v].to[c].unwrap();
            }
            nodes[v].cost = nodes[v].cost.min(costs[i]);
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
            // 2. build links of "to" such that v --> to
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
                let u = nodes[v].link.unwrap();
                if nodes[u].cost == i32::MAX {
                    nodes[v].link_terminal = nodes[u].link_terminal;
                } else {
                    nodes[v].link_terminal = nodes[v].link;
                }
            }
            for c in 0..A {
                if let Some(to) = nodes[v].to[c] {
                    q.push_back(to);
                }
            }
        }

        Self { nodes, root: 0 }
    }

    fn process(&self, s: &Vec<u8>) -> i32 {
        let n = s.len();
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        let mut v = self.root;
        for i in 0..n {
            let ch = s[i];
            let c = (ch - b'a') as usize;
            v = self.nodes[v].go[c];
            if v == self.root {
                return -1;
            }

            let mut u = v;
            while u != self.root {
                let suff = self.nodes[u].len;
                let mut ndp = dp[i + 1 - suff];
                if ndp < i32::MAX && self.nodes[u].cost < i32::MAX {
                    ndp += self.nodes[u].cost;
                    dp[i + 1] = dp[i + 1].min(ndp);
                }
                let next = self.nodes[u].link_terminal;
                if next.is_none() {
                    break;
                }
                u = next.unwrap();
            }
        }
        let ans = dp[n];
        if ans == i32::MAX {
            return -1;
        }
        ans
    }
}

impl Solution {
    pub fn minimum_cost(s: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
        let s = to_u8(&s);

        // O(N * sqrt(sum len words))
        let aho = Aho::new(words, costs);
        aho.process(&s)
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
    #[case("abcdef", vec!["abdef".into(),"abc".into(),"d".into(),"def".into(),"ef".into()], vec![100,1,1,10,5], 7)]
    #[case("aaaa", vec!["z".into(),"zz".into(),"zzz".into()], vec![1,10,100], -1)]
    fn case(
        #[case] target: String,
        #[case] words: Vec<String>,
        #[case] costs: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::minimum_cost(target, words, costs);
        assert_eq!(actual, expected);
    }
}
