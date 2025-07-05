//! Solution for https://leetcode.com/problems/maximum-points-after-collecting-coins-from-all-nodes
//! 2920. Maximum Points After Collecting Coins From All Nodes

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

struct Solver {
    tree: Tree,
    dp: Vec<Vec<i32>>,
    coins: Vec<i32>,
    k: i32,
}

impl Solver {
    fn new(tree: Tree, coins: Vec<i32>, k: i32) -> Self {
        let mut log = 0;
        let max_coin = *coins.iter().max().unwrap();
        while (1 << log) < max_coin {
            log += 1;
        }
        log += 1;
        let n = tree.n;
        Self {
            dp: vec![vec![i32::MIN; log]; tree.n],
            tree,
            coins,
            k,
        }
    }

    fn solve(&mut self) -> i32 {
        self.dfs(self.tree.root);
        self.dp[self.tree.root][0]
    }

    fn dfs(&mut self, v: usize) {
        let len_to = self.tree.g[v].len();
        for i in 0..len_to {
            let to = self.tree.g[v][i];
            if to == self.tree.par[v] {
                continue;
            }
            self.dfs(to);
        }

        let mut coins = self.coins[v];
        let log = self.dp[v].len();
        for times in 0..log {
            let mut cur = coins - self.k;
            for i in 0..len_to {
                let to = self.tree.g[v][i];
                if to != self.tree.par[v] {
                    cur += self.dp[to][times];
                }
            }
            self.dp[v][times] = self.dp[v][times].max(cur);

            cur = coins / 2;
            for i in 0..len_to {
                let to = self.tree.g[v][i];
                if to != self.tree.par[v] {
                    cur += self.dp[to][(times + 1).min(log - 1)];
                }
            }
            self.dp[v][times] = self.dp[v][times].max(cur);

            coins /= 2;
        }
    }
}

impl Solution {
    pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
        let n = coins.len();
        let tree = Tree::new_from_i32_edges(n, &edges, 0);
        let mut solver = Solver::new(tree, coins, k);
        solver.solve()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1],vec![1,2],vec![2,3]], vec![10,10,3,3], 5, 11)]
    #[case(vec![vec![0,1],vec![0,2]], vec![8,4,4], 0, 16)]
    fn case(
        #[case] edges: Vec<Vec<i32>>,
        #[case] coins: Vec<i32>,
        #[case] k: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::maximum_points(edges, coins, k);
        assert_eq!(actual, expected);
    }
}
