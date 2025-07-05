//! Solution for https://leetcode.com/problems/stone-game-iv
//! 1510. Stone Game IV

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;

        let mut dp = vec![0; n + 1];
        dp[0] = -1;
        for i in 1..=n {
            dp[i] = -1;
            for j in 1..=i {
                if j * j > i {
                    continue;
                }
                if dp[i - j * j] == -1 {
                    dp[i] = 1;
                    break;
                }
            }
        }
        dp[n] == 1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, true)]
    #[case(2, false)]
    #[case(4, true)]
    fn case(#[case] n: i32, #[case] expected: bool) {
        let actual = Solution::winner_square_game(n);
        assert_eq!(actual, expected);
    }
}
