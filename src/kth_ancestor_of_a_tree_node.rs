//! Solution for https://leetcode.com/problems/kth-ancestor-of-a-tree-node
//! 1483. Kth Ancestor of a Tree Node

struct TreeAncestor {
    par: Vec<Vec<usize>>,
    log: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let n = (n + 1) as usize;
        let mut log = 0;
        while (1 << log) < n {
            log += 1;
        }
        log += 1;

        let mut par = vec![vec![0; log]; n];
        for i in 1..n {
            par[i][0] = (parent[i - 1] + 1) as usize;
        }
        for l in 1..log {
            for i in 0..n {
                par[i][l] = par[par[i][l - 1]][l - 1];
            }
        }

        Self { par, log }
    }

    fn get_kth_ancestor(&self, v: i32, k: i32) -> i32 {
        let mut v = (v + 1) as usize;
        let mut k = k as usize;
        for l in (0..self.log).rev() {
            if (1 << l) <= k {
                v = self.par[v][l];
                k -= 1 << l;
            }
        }
        v as i32 - 1
    }
}

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
