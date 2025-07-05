//! Solution for https://leetcode.com/problems/rank-transform-of-a-matrix
//! 1632. Rank Transform of a Matrix

use std::collections::HashMap;

struct DSU {
    p: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self { p: vec![-1; n] }
    }

    fn get(&mut self, x: usize) -> usize {
        if self.p[x] < 0 {
            x
        } else {
            self.p[x] = self.get(self.p[x] as usize) as i32;
            self.p[x] as usize
        }
    }

    fn merge(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.get(x);
        y = self.get(y);
        if x == y {
            return false;
        }
        if -self.p[x] < -self.p[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.p[x] += self.p[y];
        self.p[y] = x as i32;
        true
    }

    fn comp_size(&mut self, x: usize) -> usize {
        let root = self.get(x);
        (-self.p[root]) as usize
    }
}

fn dfs(v: usize, g: &Vec<Vec<usize>>, was: &mut Vec<bool>, top: &mut Vec<usize>) {
    assert!(!was[v]);
    was[v] = true;
    for &to in &g[v] {
        if !was[to] {
            dfs(to, g, was, top);
        }
    }
    top.push(v);
}

impl Solution {
    pub fn matrix_rank_transform(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = a.len();
        let m = a[0].len();

        let mut dsu = DSU::new(n * m);
        for i in 0..n {
            let mut col_of = HashMap::<i32, usize>::new();
            for j in 0..m {
                let v = a[i][j];
                if let Some(jj) = col_of.get(&v) {
                    dsu.merge(i * m + j, i * m + jj);
                } else {
                    col_of.insert(v, j);
                }
            }
        }
        for j in 0..m {
            let mut row_of = HashMap::<i32, usize>::new();
            for i in 0..n {
                let v = a[i][j];
                if let Some(ii) = row_of.get(&v) {
                    dsu.merge(i * m + j, ii * m + j);
                } else {
                    row_of.insert(v, i);
                }
            }
        }

        let mut g = vec![Vec::new(); n * m];
        let mut add_edge = |from: usize, to: usize| {
            g[dsu.get(from)].push(dsu.get(to));
        };
        for i in 0..n {
            let mut js: Vec<_> = (0..m).collect();
            js.sort_by_key(|&j| a[i][j]);
            for k in 0..js.len() - 1 {
                if a[i][js[k]] != a[i][js[k + 1]] {
                    add_edge(i * m + js[k], i * m + js[k + 1]);
                }
            }
        }
        for j in 0..m {
            let mut is: Vec<_> = (0..n).collect();
            is.sort_by_key(|&i| a[i][j]);
            for k in 0..is.len() - 1 {
                if a[is[k]][j] != a[is[k + 1]][j] {
                    add_edge(is[k] * m + j, is[k + 1] * m + j);
                }
            }
        }

        let mut was = vec![false; n * m];
        let mut top = Vec::new();
        for v in 0..n * m {
            if !was[v] && dsu.get(v) == v {
                dfs(v, &g, &mut was, &mut top);
            }
        }
        top.reverse();

        let mut rank = vec![1; n * m];
        for &v in &top {
            for &to in &g[v] {
                rank[to] = rank[to].max(rank[v] + 1);
            }
        }
        let mut ans = vec![vec![0; m]; n];
        for v in 0..n * m {
            let root = dsu.get(v);
            let true_rank = rank[root];
            ans[v / m][v % m] = true_rank as i32;
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
    #[case(vec![vec![1,2],vec![3,4]], vec![vec![1,2],vec![2,3]])]
    #[case(vec![vec![7,7],vec![7,7]], vec![vec![1,1],vec![1,1]])]
    #[case(vec![vec![20,-21,14],vec![-19,4,19],vec![22,-47,24],vec![-19,4,19]], vec![vec![4,2,3],vec![1,3,4],vec![5,1,6],vec![1,3,4]])]
    fn case(#[case] matrix: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::matrix_rank_transform(matrix);
        assert_eq!(actual, expected);
    }
}
