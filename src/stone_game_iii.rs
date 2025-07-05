//! Solution for https://leetcode.com/problems/stone-game-iii
//! 1406. Stone Game III

impl Solution {
    pub fn stone_game_iii(a: Vec<i32>) -> String {
        let n = a.len();

        let mut dp = vec![i32::MIN; n + 1];
        dp[n] = 0;
        for i in (0..n).rev() {
            let mut take = 0;
            for j in i..(i + 3).min(n) {
                take += a[j];
                dp[i] = dp[i].max(take - dp[j + 1]);
            }
        }
        match dp[0].signum() {
            -1 => return "Bob".to_string(),
            1 => return "Alice".to_string(),
            0 => return "Tie".to_string(),
            _ => panic!(),
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,7], "Bob")]
    #[case(vec![1,2,3,-9], "Alice")]
    #[case(vec![1,2,3,6], "Tie")]
    fn case(#[case] stone_value: Vec<i32>, #[case] expected: String) {
        let actual = Solution::stone_game_iii(stone_value);
        assert_eq!(actual, expected);
    }
}
