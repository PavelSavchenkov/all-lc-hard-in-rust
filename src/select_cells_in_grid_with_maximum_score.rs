//! Solution for https://leetcode.com/problems/select-cells-in-grid-with-maximum-score
//! 3276. Select Cells in Grid With Maximum Score

impl Solution {
    pub fn max_score(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let mut M = 0;
        for i in 0..n {
            for j in 0..m {
                M = M.max(g[i][j]);
            }
        }
        let M = M as usize;

        let mut ans = 0;
        let mut dp = vec![0; 1 << n];
        for x in 0..=M {
            let mut rows = Vec::new();
            for i in 0..n {
                for j in 0..m {
                    if g[i][j] as usize == x {
                        rows.push(i);
                        break;
                    }
                }
            }
            for mask in (0..1 << n).rev() {
                let cur_dp = dp[mask];
                for &i in &rows {
                    let nmask = mask | (1 << i);
                    if nmask == mask {
                        continue;
                    }
                    let ndp = cur_dp + x;
                    ans = ans.max(ndp);
                    dp[nmask] = dp[nmask].max(cur_dp + x);
                }
            }
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
    #[case(vec![vec![1,2,3],vec![4,3,2],vec![1,1,1]], 8)]
    #[case(vec![vec![8,7,6],vec![8,3,2]], 15)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::max_score(grid);
        assert_eq!(actual, expected);
    }
}
