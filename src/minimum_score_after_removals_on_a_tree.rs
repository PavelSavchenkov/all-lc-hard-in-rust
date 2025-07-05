//! Solution for https://leetcode.com/problems/minimum-score-after-removals-on-a-tree
//! 2322. Minimum Score After Removals on a Tree

struct Tree {
    g: Vec<Vec<usize>>,
    tin: Vec<usize>,
    tout: Vec<usize>,
    par: Vec<usize>,
    n: usize,
    root: usize,
}

impl Tree {
    fn new(n: usize, root: usize) -> Self {
        assert!(n > 0);
        Self {
            g: vec![Vec::new(); n],
            tin: Vec::new(),
            tout: Vec::new(),
            par: Vec::new(),
            n,
            root,
        }
    }

    fn add_e(&mut self, a: usize, b: usize) {
        assert!(self.par.is_empty());
        self.g[a].push(b);
        self.g[b].push(a);
    }

    fn run_dfs_pre(&mut self) {
        self.par = vec![self.n; self.n];
        self.tin = vec![0; self.n];
        self.tout = vec![0; self.n];
        let mut timer = 0;
        self.dfs_pre(self.root, self.n, &mut timer);
    }

    fn dfs_pre(&mut self, v: usize, p: usize, timer: &mut usize) {
        self.tin[v] = *timer;
        *timer += 1;
        self.par[v] = p;

        let g_len = self.g[v].len();
        for i in 0..g_len {
            let to = self.g[v][i];
            if to == p {
                continue;
            }
            self.dfs_pre(to, v, timer);
        }

        self.tout[v] = *timer;
    }

    fn is_upper(&self, v: usize, u: usize) -> bool {
        self.tin[v] <= self.tin[u] && self.tout[u] <= self.tout[v]
    }

    fn run_dfs_xor(&self, xor: &mut Vec<i32>, nums: &Vec<i32>) {
        self.dfs_xor(self.root, xor, nums);
    }

    fn dfs_xor(&self, v: usize, xor: &mut Vec<i32>, nums: &Vec<i32>) {
        xor[v] = nums[v];
        for &to in &self.g[v] {
            if to == self.par[v] {
                continue;
            }
            self.dfs_xor(to, xor, nums);
            xor[v] ^= xor[to];
        }
    }
}

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();

        let mut N = 0;
        for i in 0..n {
            N ^= nums[i];
        }

        let mut tree = Tree::new(n, 0);
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            tree.add_e(a, b);
        }
        tree.run_dfs_pre();

        let mut xor = vec![0 as i32; n];
        tree.run_dfs_xor(&mut xor, &nums);

        let mut ans = i32::MAX;
        for a in 0..n {
            for &b in &tree.g[a] {
                if b == tree.par[a] {
                    continue;
                }
                for c in 0..n {
                    for &d in &tree.g[c] {
                        if d == tree.par[c] || d == b {
                            continue;
                        }
                        if tree.is_upper(b, d) {
                            let A = N ^ xor[b];
                            let B = xor[d];
                            let C = N ^ A ^ B;
                            let cur = A.max(B.max(C)) - A.min(B.min(C));
                            ans = ans.min(cur);
                        } else if !tree.is_upper(d, b) {
                            let A = xor[d];
                            let B = xor[b];
                            let C = N ^ A ^ B;
                            let cur = A.max(B.max(C)) - A.min(B.min(C));
                            ans = ans.min(cur);
                        }
                    }
                }
            }
        }

        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,5,5,4,11], vec![vec![0,1],vec![1,2],vec![1,3],vec![3,4]], 9)]
    #[case(vec![5,5,2,4,4,2], vec![vec![0,1],vec![1,2],vec![5,2],vec![4,3],vec![1,3]], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] edges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::minimum_score(nums, edges);
        assert_eq!(actual, expected);
    }
}
