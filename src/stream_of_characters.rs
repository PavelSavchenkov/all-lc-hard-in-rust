//! Solution for https://leetcode.com/problems/stream-of-characters
//! 1032. Stream of Characters

use std::collections::VecDeque;

const A: usize = 26;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    to: Vec<Option<usize>>,
    go: Vec<usize>,
    link: Option<usize>,
    terminal: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            to: vec![None; A],
            go: Vec::new(),
            link: None,
            terminal: false,
        }
    }
}

struct StreamChecker {
    nodes: Vec<Node>,
    v: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let words = to_u8_vec(&words);
        let mut nodes = Vec::new();
        nodes.push(Node::new());
        for word in &words {
            let mut v = 0;
            for ch in word {
                let c = (ch - b'a') as usize;
                if nodes[v].to[c].is_none() {
                    nodes[v].to[c] = Some(nodes.len());
                    nodes.push(Node::new());
                }
                v = nodes[v].to[c].unwrap();
            }
            nodes[v].terminal = true;
        }

        // suffix links and go
        let mut q = VecDeque::new();
        q.push_back(0);
        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            // link is ready, go is not
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
        for v in 1..nodes.len() {
            let from = nodes[v].link.unwrap();
            if nodes[from].terminal {
                nodes[v].terminal = true;
            }
        }

        Self { nodes, v: 0 }
    }

    fn query(&mut self, letter: char) -> bool {
        let letter = letter as u8;
        let c = (letter - b'a') as usize;
        self.v = self.nodes[self.v].go[c];
        self.nodes[self.v].terminal
    }
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */

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

#[cfg(test)]
mod tests {
    use super::*;
}
