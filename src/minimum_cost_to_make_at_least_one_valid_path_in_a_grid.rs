//! Solution for https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid
//! 1368. Minimum Cost to Make at Least One Valid Path in a Grid

use std::collections::VecDeque;

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn min_cost(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let mut dp = vec![vec![usize::MAX; m]; n];
        dp[0][0] = 0;
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        while !q.is_empty() {
            let (i, j) = q.pop_front().unwrap();
            let cur_dp = dp[i][j];
            for d in 0..4 {
                let ni = i as i32 + di[d];
                let nj = j as i32 + dj[d];
                if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                let ndp = cur_dp + (if (d as i32) != (g[i][j] - 1) { 1 } else { 0 });
                if ndp < dp[ni][nj] {
                    dp[ni][nj] = ndp;
                    q.push_back((ni, nj));
                }
            }
        }
        dp[n - 1][m - 1] as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,1,1,1],vec![2,2,2,2],vec![1,1,1,1],vec![2,2,2,2]], 3)]
    #[case(vec![vec![1,1,3],vec![3,2,2],vec![1,1,4]], 0)]
    #[case(vec![vec![1,2],vec![4,3]], 1)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_cost(grid);
        assert_eq!(actual, expected);
    }
}
