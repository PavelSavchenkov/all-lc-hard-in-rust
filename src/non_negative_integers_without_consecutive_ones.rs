//! Solution for https://leetcode.com/problems/non-negative-integers-without-consecutive-ones
//! 600. Non-negative Integers without Consecutive Ones

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let mut n = (n + 1) as usize;

        let mut digits = Vec::new();
        while n > 0 {
            digits.push(n % 2);
            n /= 2;
        }
        digits.reverse();

        let mut dp = vec![vec![vec![0; 2]; 2]; digits.len() + 1];
        dp[0][0][0] = 1;
        for i in 0..digits.len() {
            for less in 0..2 {
                for last in 0..2 {
                    let cur_dp = dp[i][less][last];
                    if cur_dp == 0 {
                        continue;
                    }
                    for next in 0..2 {
                        if last == 1 && next == 1 {
                            continue;
                        }
                        if less == 0 && next > digits[i] {
                            continue;
                        }
                        let mut nless = less;
                        if next < digits[i] {
                            nless = 1;
                        }
                        dp[i + 1][nless][next] += cur_dp;
                    }
                }
            }
        }

        dp[digits.len()][1].iter().sum::<i32>()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, 5)]
    #[case(1, 2)]
    #[case(2, 3)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::find_integers(n);
        assert_eq!(actual, expected);
    }
}
