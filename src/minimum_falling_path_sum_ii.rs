//! Solution for https://leetcode.com/problems/minimum-falling-path-sum-ii
//! 1289. Minimum Falling Path Sum II

impl Solution {
    pub fn min_falling_path_sum(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();

        let mut dp = g[0].clone();
        for i in 1..n {
            let mut ndp = vec![i32::MAX; n];
            for j in 0..n {
                for j2 in 0..n {
                    if j2 != j {
                        ndp[j] = ndp[j].min(dp[j2] + g[i][j]);
                    }
                }
            }
            dp = ndp;
        }
        *dp.iter().min().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]], 13)]
    #[case(vec![vec![7]], 7)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_falling_path_sum(grid);
        assert_eq!(actual, expected);
    }
}
