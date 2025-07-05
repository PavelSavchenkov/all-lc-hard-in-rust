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
                    nodes[v].to[c] = Some(nodes.len());
                    nodes.push(Node::new());
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
}
