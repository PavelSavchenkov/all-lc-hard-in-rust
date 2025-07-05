//! Solution for https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination
//! 1293. Shortest Path in a Grid with Obstacles Elimination

use std::collections::VecDeque;

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn shortest_path(g: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = g.len();
        let m = g[0].len();

        let mut dp = vec![vec![vec![usize::MAX; k + 1]; m]; n];
        dp[0][0][0] = 0;
        let mut q = VecDeque::new();
        q.push_back((0, 0, 0));
        while !q.is_empty() {
            let (i, j, deleted) = q.pop_front().unwrap();
            let cur_dp = dp[i][j][deleted];
            for d in 0..4 {
                let ni = i as i32 + di[d];
                let nj = j as i32 + dj[d];
                if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                let mut ndeleted = deleted;
                if g[ni][nj] == 1 {
                    ndeleted += 1;
                    if ndeleted == k + 1 {
                        continue;
                    }
                }
                if dp[ni][nj][ndeleted] > cur_dp + 1 {
                    dp[ni][nj][ndeleted] = cur_dp + 1;
                    q.push_back((ni, nj, ndeleted));
                }
            }
        }
        let ans = *dp[n - 1][m - 1].iter().min().unwrap();
        if ans == usize::MAX {
            return -1;
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
    #[case(vec![vec![0,0,0],vec![1,1,0],vec![0,0,0],vec![0,1,1],vec![0,0,0]], 1, 6)]
    #[case(vec![vec![0,1,1],vec![1,1,1],vec![1,0,0]], 1, -1)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::shortest_path(grid, k);
        assert_eq!(actual, expected);
    }
}
