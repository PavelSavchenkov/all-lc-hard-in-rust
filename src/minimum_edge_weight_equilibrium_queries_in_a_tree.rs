//! Solution for https://leetcode.com/problems/minimum-edge-weight-equilibrium-queries-in-a-tree
//! 2846. Minimum Edge Weight Equilibrium Queries in a Tree

struct TreeEdge {
    a: usize,
    b: usize,
}

#[derive(Default)]
struct Tree {
    g: Vec<Vec<usize>>,
    n: usize,
    root: usize,
    // vvv calculated data
    log_n: usize,
    par_pw: Vec<Vec<usize>>,
    par: Vec<usize>,
    tin: Vec<usize>,
    tout: Vec<usize>,
    depth: Vec<usize>,
}

impl Tree {
    fn new_from_tree_edges(n: usize, edges: &Vec<TreeEdge>, root: usize) -> Self {
        let mut g = vec![Vec::new(); n];
        for e in edges {
            g[e.a].push(e.b);
            g[e.b].push(e.a);
        }
        let mut this = Self {
            g,
            n,
            ..Default::default()
        };
        this.pre();
        this
    }

    fn new_from_i32_edges(n: usize, edges: &Vec<Vec<i32>>, root: usize) -> Self {
        let edges: Vec<_> = edges
            .iter()
            .map(|e| TreeEdge {
                a: e[0] as usize,
                b: e[1] as usize,
            })
            .collect();
        Self::new_from_tree_edges(n, &edges, root)
    }

    fn pre(&mut self) {
        self.log_n = 1;
        while (1 << self.log_n) < self.n {
            self.log_n += 1;
        }
        self.par_pw = vec![vec![self.root; self.log_n]; self.n];
        self.par = vec![self.root; self.n];
        self.tin = vec![0; self.n];
        self.tout = vec![0; self.n];
        self.depth = vec![0; self.n];
        self.run_dfs_pre();
        for l in 1..self.log_n {
            for v in 0..self.n {
                self.par_pw[v][l] = self.par_pw[self.par_pw[v][l - 1]][l - 1]
            }
        }
    }

    fn run_dfs_pre(&mut self) {
        let mut timer = 0;
        self.dfs_pre(self.root, self.root, 0, &mut timer);
    }

    fn dfs_pre(&mut self, v: usize, p: usize, depth: usize, timer: &mut usize) {
        self.depth[v] = depth;
        self.tin[v] = *timer;
        *timer += 1;
        self.par_pw[v][0] = p;
        self.par[v] = p;
        for i in 0..self.g[v].len() {
            let to = self.g[v][i];
            if to != p {
                self.dfs_pre(to, v, depth + 1, timer);
            }
        }
        self.tout[v] = *timer;
    }

    fn is_upper(&self, up: usize, down: usize) -> bool {
        self.tin[up] <= self.tin[down] && self.tout[down] <= self.tout[up]
    }

    fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] < self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        for l in (0..self.log_n).rev() {
            let aa = self.par_pw[a][l];
            if !self.is_upper(aa, b) {
                a = aa;
            }
        }
        if !self.is_upper(a, b) {
            self.par[a]
        } else {
            a
        }
    }
}

const MAXW: usize = 26;

impl Solution {
    pub fn min_operations_queries(
        n: i32,
        edges: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let tree = Tree::new_from_i32_edges(n, &edges, 0);

        let mut ws = vec![0; n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let w = e[2] as usize;
            assert!(w <= MAXW);
            if tree.depth[a] > tree.depth[b] {
                ws[a] = w - 1;
            } else {
                ws[b] = w - 1;
            }
        }

        let mut cnt_w = vec![vec![0; MAXW]; n];
        let mut order: Vec<_> = (0..n).collect();
        order.sort_by_key(|&i| tree.depth[i]);
        for &i in &order {
            if i == 0 {
                continue;
            }
            for w in 0..MAXW {
                cnt_w[i][w] = cnt_w[tree.par[i]][w];
            }
            cnt_w[i][ws[i]] += 1;
        }

        let mut ans = Vec::new();
        for q in &queries {
            let a = q[0] as usize;
            let b = q[1] as usize;
            let c = tree.lca(a, b);
            let mut max_cnt = 0;
            for w in 0..MAXW {
                let cnt = cnt_w[a][w] + cnt_w[b][w] - 2 * cnt_w[c][w];
                max_cnt = max_cnt.max(cnt);
            }
            let dist = tree.depth[a] + tree.depth[b] - 2 * tree.depth[c];
            let cur = dist - max_cnt;
            ans.push(cur as i32);
        }
        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(7, vec![vec![0,1,1],vec![1,2,1],vec![2,3,1],vec![3,4,2],vec![4,5,2],vec![5,6,2]], vec![vec![0,3],vec![3,6],vec![2,6],vec![0,6]], vec![0,0,1,3])]
    #[case(8, vec![vec![1,2,6],vec![1,3,4],vec![2,4,6],vec![2,5,3],vec![3,6,6],vec![3,0,8],vec![7,0,2]], vec![vec![4,6],vec![0,4],vec![6,5],vec![7,4]], vec![1,2,2,3])]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] queries: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::min_operations_queries(n, edges, queries);
        assert_eq!(actual, expected);
    }
}
