//! Solution for https://leetcode.com/problems/stone-game-viii
//! 1872. Stone Game VIII

impl Solution {
    pub fn stone_game_viii(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut pref = vec![0; n];
        pref[0] = a[0];
        for i in 1..n {
            pref[i] = pref[i - 1] + a[i];
        }

        let mut dp = vec![0; n];
        let mut max = pref[n - 1] - dp[n - 1];
        for i in (0..n - 1).rev() {
            dp[i] = max;
            max = max.max(pref[i] - dp[i]);
        }
        dp[0]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![-1,2,-3,4,-5], 5)]
    #[case(vec![7,-6,5,10,5,-2,-6], 13)]
    #[case(vec![-10,-12], -22)]
    fn case(#[case] stones: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::stone_game_viii(stones);
        assert_eq!(actual, expected);
    }
}
