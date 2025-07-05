//! Solution for https://leetcode.com/problems/minimum-edge-reversals-so-every-node-is-reachable
//! 2858. Minimum Edge Reversals So Every Node Is Reachable

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

fn dfs(v: usize, tree: &Tree, is_up: &Vec<bool>, ans: &mut Vec<i32>) {
    for &to in &tree.g[v] {
        if to == tree.par[v] {
            continue;
        }
        ans[to] = ans[v];
        if is_up[to] {
            ans[to] -= 1;
        } else {
            ans[to] += 1;
        }
        dfs(to, tree, is_up, ans);
    }
}

impl Solution {
    pub fn min_edge_reversals(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let tree = Tree::new_from_i32_edges(n, &edges, 0);

        let mut is_up = vec![false; n];
        let mut cnt_up = 0;
        for e in &edges {
            let from = e[0] as usize;
            let to = e[1] as usize;
            if to == tree.par[from] {
                is_up[from] = true;
                cnt_up += 1;
            }
        }

        let mut ans = vec![0; n];
        ans[0] = cnt_up;
        dfs(0, &tree, &is_up, &mut ans);
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
    #[case(4, vec![vec![2,0],vec![2,1],vec![1,3]], vec![1,1,0,2])]
    #[case(3, vec![vec![1,2],vec![2,0]], vec![2,0,1])]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::min_edge_reversals(n, edges);
        assert_eq!(actual, expected);
    }
}
