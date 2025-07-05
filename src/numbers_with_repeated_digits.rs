//! Solution for https://leetcode.com/problems/numbers-with-repeated-digits
//! 1012. Numbers With Repeated Digits

impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let all = n;
        let mut n = (n + 1) as usize;
        let mut digits = Vec::new();
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();

        // dp[i][mask][all_zeros][less]
        let mut dp = vec![vec![vec![vec![0; 2]; 2]; 1 << 10]; digits.len() + 1];
        dp[0][0][1][0] = 1;
        for i in 0..digits.len() {
            for mask in 0..1 << 10 {
                for all_zeros in 0..2 {
                    for less in 0..2 {
                        let cur_dp = dp[i][mask][all_zeros][less];
                        if cur_dp == 0 {
                            continue;
                        }
                        for next in 0..10 {
                            if all_zeros == 1 && next == 0 {
                                assert!(mask == 0);
                                dp[i + 1][mask][1][1] += cur_dp;
                                continue;
                            }
                            if ((mask >> next) & 1) == 1 {
                                continue;
                            }
                            if less == 0 && next > digits[i] {
                                continue;
                            }
                            let mut nless = less;
                            if next < digits[i] {
                                nless = 1;
                            }
                            let nmask = mask ^ (1 << next);
                            dp[i + 1][nmask][0][nless] += cur_dp;
                        }
                    }
                }
            }
        }

        let mut bad = 0;
        for mask in 0..1 << 10 {
            bad += dp[digits.len()][mask][0][1];
        }
        let ans = all - bad;
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
    #[case(20, 1)]
    #[case(100, 10)]
    #[case(1000, 262)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::num_dup_digits_at_most_n(n);
        assert_eq!(actual, expected);
    }
}
