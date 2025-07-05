//! Solution for https://leetcode.com/problems/maximum-score-from-grid-operations
//! 3225. Maximum Score From Grid Operations

impl Solution {
    pub fn maximum_score(g: Vec<Vec<i32>>) -> i64 {
        let n = g.len();

        let mut pref = vec![vec![0; n]; n + 1];
        for j in 0..n {
            for i in 0..n {
                pref[i + 1][j] = pref[i][j] + g[i][j] as i64;
            }
        }

        let mut ans = 0;
        let mut dp = vec![vec![i64::MIN; n + 1]; n + 1];
        dp[0][n] = 0;
        for j in 0..n {
            let mut ndp = vec![vec![i64::MIN; n + 1]; n + 1];
            for h1 in 0..=n {
                for h2 in 0..=n - h1 {
                    let cur_dp = dp[h1][h2];
                    if cur_dp == i64::MIN {
                        continue;
                    }
                    for h in 0..=n {
                        let mut cur = cur_dp;
                        let mut new_h2 = 0;
                        if h < h1 {
                            cur += pref[h1][j] - pref[h][j];
                            new_h2 = h1 - h;
                        } else if h > h1 + h2 {
                            assert!(j > 0);
                            cur += pref[h][j - 1] - pref[h1 + h2][j - 1];
                        }
                        ndp[h][new_h2] = ndp[h][new_h2].max(cur);
                        ans = ans.max(cur);
                    }
                }
            }
            dp = ndp;
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
    #[case(vec![vec![0,0,0,0,0],vec![0,0,3,0,0],vec![0,1,0,0,0],vec![5,0,0,3,0],vec![0,0,0,0,2]], 11)]
    #[case(vec![vec![10,9,0,0,15],vec![7,1,0,8,0],vec![5,20,0,11,0],vec![0,0,0,1,2],vec![8,12,1,10,3]], 94)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i64) {
        let actual = Solution::maximum_score(grid);
        assert_eq!(actual, expected);
    }
}
