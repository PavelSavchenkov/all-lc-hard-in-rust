//! Solution for https://leetcode.com/problems/dungeon-game
//! 174. Dungeon Game

impl Solution {
    pub fn calculate_minimum_hp(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let can = |hp: i32| -> bool {
            let mut dp = vec![vec![0; m]; n];
            dp[0][0] = hp + g[0][0];
            for i in 0..n {
                for j in 0..m {
                    let base = dp[i][j];
                    if base <= 0 {
                        continue;
                    }
                    if i + 1 < n {
                        dp[i + 1][j] = dp[i + 1][j].max(base + g[i + 1][j]);
                    }
                    if j + 1 < m {
                        dp[i][j + 1] = dp[i][j + 1].max(base + g[i][j + 1]);
                    }
                }
            }
            dp[n - 1][m - 1] > 0
        };

        let mut L = 0 as i32;
        let mut R = 1000 * (n + m) as i32 + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if can(M) {
                R = M
            } else {
                L = M
            }
        }
        R as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![-2,-3,3],vec![-5,-10,1],vec![10,30,-5]], 7)]
    #[case(vec![vec![0]], 1)]
    fn case(#[case] dungeon: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::calculate_minimum_hp(dungeon);
        assert_eq!(actual, expected);
    }
}
