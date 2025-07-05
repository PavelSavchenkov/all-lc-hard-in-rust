//! Solution for https://leetcode.com/problems/minimize-the-total-price-of-the-trips
//! 2646. Minimize the Total Price of the Trips

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

#[derive(Default)]
struct Solver {
    tree: Tree,
    sum: Vec<i32>,
    price: Vec<i64>,
}

impl Solver {
    fn new(n: usize, edges: &Vec<Vec<i32>>) -> Self {
        Self {
            tree: Tree::new_from_i32_edges(n, &edges, 0),
            ..Default::default()
        }
    }

    fn run_dfs_sum(&mut self) {
        self.dfs_sum(0);
    }

    fn dfs_sum(&mut self, v: usize) {
        for i in 0..self.tree.g[v].len() {
            let to = self.tree.g[v][i];
            if to == self.tree.par[v] {
                continue;
            }
            self.dfs_sum(to);
            self.sum[v] += self.sum[to];
        }
    }

    fn run_dp_price(&self) -> i64 {
        let mut dp = vec![vec![0; 2]; self.tree.n];
        self.dfs_dp(0, &mut dp);
        *dp[0].iter().min().unwrap()
    }

    fn dfs_dp(&self, v: usize, dp: &mut Vec<Vec<i64>>) {
        for &to in &self.tree.g[v] {
            if to == self.tree.par[v] {
                continue;
            }
            self.dfs_dp(to, dp);
            dp[v][1] += dp[to][0];
            dp[v][0] += *dp[to].iter().min().unwrap();
        }
        dp[v][1] += self.price[v] / 2;
        dp[v][0] += self.price[v];
    }
}

impl Solution {
    pub fn minimum_total_price(
        n: i32,
        edges: Vec<Vec<i32>>,
        price: Vec<i32>,
        trips: Vec<Vec<i32>>,
    ) -> i32 {
        let n = n as usize;
        let mut solver = Solver::new(n, &edges);

        solver.sum = vec![0; n];
        for trip in &trips {
            let a = trip[0] as usize;
            let b = trip[1] as usize;
            let c = solver.tree.lca(a, b);
            solver.sum[a] += 1;
            solver.sum[b] += 1;
            solver.sum[c] -= 1;
            let par_c = solver.tree.par[c];
            if par_c != c {
                solver.sum[par_c] -= 1;
            }
        }
        solver.run_dfs_sum();

        solver.price = vec![0; n];
        for i in 0..n {
            solver.price[i] = price[i] as i64 * solver.sum[i] as i64;
        }

        let ans = solver.run_dp_price();
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
    #[case(4, vec![vec![0,1],vec![1,2],vec![1,3]], vec![2,2,10,6], vec![vec![0,3],vec![2,1],vec![2,3]], 23)]
    #[case(2, vec![vec![0,1]], vec![2,2], vec![vec![0,0]], 1)]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] price: Vec<i32>,
        #[case] trips: Vec<Vec<i32>>,
        #[case] expected: i32,
    ) {
        let actual = Solution::minimum_total_price(n, edges, price, trips);
        assert_eq!(actual, expected);
    }
}
