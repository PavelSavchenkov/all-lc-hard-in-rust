//! Solution for https://leetcode.com/problems/modify-graph-edge-weights
//! 2699. Modify Graph Edge Weights

const INF: i64 = i64::pow(10, 10);

struct Graph {
    g: Vec<Vec<i64>>,
    n: usize,
    var_edges: Vec<(usize, usize)>,
    src: usize,
    dst: usize,
    target_dist: i64,
    dist: Vec<Vec<i64>>,
}

impl Graph {
    fn new(n: usize, edges: &Vec<Vec<i32>>, src: usize, dst: usize, target_dist: i64) -> Self {
        let mut g = vec![vec![INF; n]; n];
        let mut var_edges = Vec::new();
        for e in edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let mut w = e[2] as i64;
            if w == -1 {
                var_edges.push((a, b));
                w = 1;
            }
            g[a][b] = w;
            g[b][a] = w;
        }
        for i in 0..n {
            g[i][i] = 0;
        }
        Self {
            g,
            n,
            var_edges,
            src,
            dst,
            target_dist,
            dist: vec![vec![INF; n]; n],
        }
    }

    fn recalc_dist(&mut self) {
        for i in 0..self.n {
            for j in 0..self.n {
                self.dist[i][j] = self.g[i][j];
            }
        }

        for k in 0..self.n {
            for i in 0..self.n {
                for j in 0..self.n {
                    self.dist[i][j] = self.dist[i][j].min(self.dist[i][k] + self.dist[k][j]);
                }
            }
        }
    }

    fn actual_dist(&self) -> i64 {
        self.dist[self.src][self.dst]
    }

    fn set_edge(&mut self, a: usize, b: usize, w: i64) {
        self.g[a][b] = w;
        self.g[b][a] = w;
    }

    fn can_target_dist(&mut self) -> bool {
        self.recalc_dist();

        if self.actual_dist() > self.target_dist {
            return false;
        }
        if self.actual_dist() == self.target_dist {
            return true;
        }

        let mut L = 0;
        let mut R = self.var_edges.len() + 1;
        while L + 1 != R {
            let M = (L + R) / 2;

            for i in 0..self.var_edges.len() {
                let (a, b) = self.var_edges[i];
                self.set_edge(a, b, if i < M { self.target_dist + 1 } else { 1 });
            }
            self.recalc_dist();
            if self.actual_dist() < self.target_dist {
                L = M
            } else {
                R = M
            }
        }
        if L == self.var_edges.len() {
            return false;
        }
        for i in 0..self.var_edges.len() {
            let (a, b) = self.var_edges[i];
            self.set_edge(a, b, if i < L { self.target_dist + 1 } else { 1 });
        }
        self.recalc_dist();
        let prev_dist = self.actual_dist();
        {
            let (a, b) = self.var_edges[L];
            self.set_edge(a, b, 1 + self.target_dist - prev_dist);
        }
        return true;
    }
}

impl Solution {
    pub fn modified_graph_edges(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut g = Graph::new(
            n,
            &edges,
            source as usize,
            destination as usize,
            target as i64,
        );

        if !g.can_target_dist() {
            return Vec::new();
        }

        let mut ans = Vec::new();
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let w = g.g[a][b];
            ans.push(vec![a as i32, b as i32, w as i32]);
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
    #[case(5, vec![vec![4,1,-1],vec![2,0,-1],vec![0,3,-1],vec![4,3,-1]], 0, 1, 5, vec![vec![4,1,1],vec![2,0,1],vec![0,3,3],vec![4,3,1]])]
    #[case(3, vec![vec![0,1,-1],vec![0,2,5]], 0, 2, 6, vec![])]
    #[case(4, vec![vec![1,0,4],vec![1,2,3],vec![2,3,5],vec![0,3,-1]], 0, 2, 6, vec![vec![1,0,4],vec![1,2,3],vec![2,3,5],vec![0,3,1]])]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] source: i32,
        #[case] destination: i32,
        #[case] target: i32,
        #[case] expected: Vec<Vec<i32>>,
    ) {
        let actual = Solution::modified_graph_edges(n, edges, source, destination, target);
        assert_eq!(actual, expected);
    }
}
