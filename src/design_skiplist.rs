//! Solution for https://leetcode.com/problems/design-skiplist
//! 1206. Design Skiplist

use rand::{rngs::ThreadRng, Rng};

#[derive(Default, Clone)]
struct Node {
    val: i32,
    right: Option<usize>,
    bottom: Option<usize>,
}

impl Node {
    fn new_with_val(val: i32) -> Self {
        Self {
            val,
            ..Default::default()
        }
    }

    fn new() -> Self {
        Self::new_with_val(-1)
    }
}

struct Skiplist {
    heads: Vec<usize>,
    nodes: Vec<Node>,
    rng: ThreadRng,
    p: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {
    fn new() -> Self {
        Self {
            heads: vec![0; 1],
            nodes: vec![Node::new(); 1],
            rng: rand::thread_rng(),
            p: 0.5,
        }
    }

    // the rightmost with val <= target at every layer
    fn search_node(&self, target: i32) -> Vec<usize> {
        let mut v = *self.heads.last().unwrap();
        let mut vs = Vec::new();
        loop {
            while let Some(r) = self.nodes[v].right {
                if self.nodes[r].val <= target {
                    v = r;
                } else {
                    break;
                }
            }
            vs.push(v);
            if let Some(b) = self.nodes[v].bottom {
                v = b;
            } else {
                break;
            }
        }
        vs.reverse();
        vs
    }

    fn search(&self, target: i32) -> bool {
        let vs = self.search_node(target);
        let v = vs[0];
        self.nodes[v].val == target
    }

    fn insert_to_the_right(&mut self, v: usize, val: i32) -> usize {
        let new_id = self.nodes.len();
        let mut node = Node::new_with_val(val);
        node.right = self.nodes[v].right;
        self.nodes[v].right = Some(new_id);
        self.nodes.push(node);
        new_id
    }

    fn coin_flip_success(&mut self) -> bool {
        self.rng.gen::<f64>() < self.p
    }

    fn add(&mut self, val: i32) {
        let mut vs = self.search_node(val);
        assert!(vs.len() == self.heads.len());
        let mut bottom = None;
        let mut i = 0;
        loop {
            let new_id = self.insert_to_the_right(vs[i], val);
            self.nodes[new_id].bottom = bottom;
            bottom = Some(new_id);
            if !self.coin_flip_success() {
                break;
            }
            i += 1;
            // create a new layer
            if i == vs.len() {
                let new_id = self.nodes.len();
                let mut node = Node::new();
                node.bottom = self.heads.last().copied();
                self.nodes.push(node);

                self.heads.push(new_id);
                vs.push(new_id);
            }
        }
    }

    fn remove_to_the_right(&mut self, v: usize) -> Option<usize> {
        if let Some(r) = self.nodes[v].right {
            let new_right = self.nodes[r].right;
            self.nodes[v].right = new_right;
            return Some(r);
        }
        None
    }

    fn erase(&mut self, val: i32) -> bool {
        let vs = self.search_node(val - 1);
        if let Some(r) = self.nodes[vs[0]].right {
            if self.nodes[r].val != val {
                return false;
            }
        } else {
            return false;
        }
        let mut bottom = None;
        for &v in &vs {
            let mut removed = None;
            if let Some(r) = self.nodes[v].right {
                if self.nodes[r].val == val {
                    if self.nodes[r].bottom == bottom {
                        removed = self.remove_to_the_right(v);
                    }
                }
            }
            if removed.is_none() {
                break;
            }
            bottom = removed;
        }

        while !self.heads.is_empty() && self.nodes[*self.heads.last().unwrap()].right.is_none() {
            self.heads.pop();
        }

        true
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
