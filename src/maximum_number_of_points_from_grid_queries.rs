//! Solution for https://leetcode.com/problems/maximum-number-of-points-from-grid-queries
//! 2503. Maximum Number of Points From Grid Queries

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
}

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn max_points(g: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let n = g.len();
        let m = g[0].len();

        let mut ij = Vec::new();
        for i in 0..n {
            for j in 0..m {
                ij.push((i, j));
            }
        }
        ij.sort_by_key(|&(i, j)| g[i][j]);

        let mut qs: Vec<_> = (0..queries.len()).collect();
        qs.sort_by_key(|&i| queries[i]);

        let mut ans = vec![0; qs.len()];
        let mut dsu = DSU::new(n * m);
        let mut ptr_ij = 0;
        for ptr_qs in 0..qs.len() {
            while ptr_ij < ij.len() {
                let (i, j) = ij[ptr_ij];
                if g[i][j] < queries[qs[ptr_qs]] {
                    ptr_ij += 1;
                    let v = i * m + j;
                    for k in 0..4 {
                        let ni = i as i32 + di[k];
                        let nj = j as i32 + dj[k];
                        if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                            continue;
                        }
                        let ni = ni as usize;
                        let nj = nj as usize;
                        if g[ni][nj] > g[i][j] {
                            continue;
                        }
                        let u = ni * m + nj;
                        dsu.merge(v, u);
                    }
                } else {
                    break;
                }
            }
            let root = dsu.get(0);
            if g[0][0] < queries[qs[ptr_qs]] {
                ans[qs[ptr_qs]] = -dsu.p[root];
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
    #[case(vec![vec![1,2,3],vec![2,5,7],vec![3,5,1]], vec![5,6,2], vec![5,8,1])]
    #[case(vec![vec![5,2,1],vec![1,1,2]], vec![3], vec![0])]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] queries: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::max_points(grid, queries);
        assert_eq!(actual, expected);
    }
}
