//! Solution for https://leetcode.com/problems/stone-game-v
//! 1563. Stone Game V

impl Solution {
    pub fn stone_game_v(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + a[i];
        }

        let mut dp = vec![vec![0; n]; n];
        for len in 2..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                let mut ndp = 0;
                for m in l..r {
                    let left = pref[m + 1] - pref[l];
                    let right = pref[r + 1] - pref[m + 1];
                    if left <= right {
                        ndp = ndp.max(left + dp[l][m]);
                    }
                    if right <= left {
                        ndp = ndp.max(right + dp[m + 1][r]);
                    }
                }
                dp[l][r] = ndp;
            }
        }
        dp[0][n - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![6,2,3,4,5,5], 18)]
    #[case(vec![7,7,7,7,7,7,7], 28)]
    #[case(vec![4], 0)]
    fn case(#[case] stone_value: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::stone_game_v(stone_value);
        assert_eq!(actual, expected);
    }
}
