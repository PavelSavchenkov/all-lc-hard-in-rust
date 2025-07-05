//! Solution for https://leetcode.com/problems/count-valid-paths-in-a-tree
//! 2867. Count Valid Paths in a Tree

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

impl Solution {
    pub fn count_paths(n: i32, mut edges: Vec<Vec<i32>>) -> i64 {
        for i in 0..edges.len() {
            for j in 0..edges[i].len() {
                edges[i][j] -= 1;
            }
        }

        let n = n as usize;
        let tree = Tree::new_from_i32_edges(n, &edges, 0);

        let mut is_prime = vec![true; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        for p in 2..=n {
            if is_prime[p] {
                for x in (p + p..=n).step_by(p) {
                    is_prime[x] = false;
                }
            }
        }

        let mut by_depth: Vec<usize> = (0..n).collect();
        by_depth.sort_by_key(|&v| tree.depth[v]);
        by_depth.reverse();

        let mut dp = vec![vec![0; 2]; n];
        let mut ans: i64 = 0;
        for &v in &by_depth {
            let marked = is_prime[v + 1];
            let p = tree.par[v];
            if marked {
                dp[v][0] = 0;
                dp[v][1] = 1;
                for &to in &tree.g[v] {
                    if to != p {
                        dp[v][1] += dp[to][0];
                    }
                }
            } else {
                dp[v][0] = 1;
                dp[v][1] = 0;
                for &to in &tree.g[v] {
                    if to != p {
                        for c in 0..2 {
                            dp[v][c] += dp[to][c];
                        }
                    }
                }
            }

            // eprintln!("v = {}, dp0 = {}, dp1 = {}", v + 1, dp[v][0], dp[v][1]);

            let mut sum_pref: i64 = 0;
            let mut sum_all: i64 = 0;
            for &to in &tree.g[v] {
                if to != p {
                    sum_all += dp[to][0] as i64;
                }
            }
            for &to in &tree.g[v] {
                if to == p {
                    continue;
                }
                if marked {
                    ans += (sum_pref + 1) * dp[to][0] as i64;
                } else {
                    ans += (sum_all - dp[to][0] as i64 + 1) * dp[to][1] as i64;
                }
                // eprintln!("to = {}, sum_pref = {}, ans = {}", to + 1, sum_pref, ans);
                sum_pref += dp[to][0] as i64;
            }
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
    #[case(5, vec![vec![1,2],vec![1,3],vec![2,4],vec![2,5]], 4)]
    #[case(6, vec![vec![1,2],vec![1,3],vec![2,4],vec![3,5],vec![3,6]], 6)]
    #[case(4, vec![vec![1,2],vec![4,1],vec![3, 4]], 4)]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: i64) {
        let actual = Solution::count_paths(n, edges);
        assert_eq!(actual, expected);
    }
}
