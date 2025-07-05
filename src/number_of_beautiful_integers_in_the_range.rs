//! Solution for https://leetcode.com/problems/number-of-beautiful-integers-in-the-range
//! 2827. Number of Beautiful Integers in the Range

fn solve(mut n: usize, k: usize) -> u32 {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    let len = digits.len();

    // i, diff, rem, less, only_zeros
    let mut dp = vec![vec![vec![vec![vec![0; 2]; 2]; k]; 2 * len + 1]; len + 1];
    dp[0][len][0][0][1] = 1;
    for i in 0..len {
        for diff in 0..=2 * len {
            for rem in 0..k {
                for less in 0..2 {
                    for only_zeros in 0..2 {
                        let cur_dp = dp[i][diff][rem][less][only_zeros];
                        if cur_dp == 0 {
                            continue;
                        }
                        for dig in 0..10 {
                            if less == 0 && dig > digits[i] {
                                continue;
                            }
                            if dig == 0 && only_zeros == 1 {
                                assert!(diff == len);
                                assert!(rem == 0);
                                dp[i + 1][diff][rem][1][1] += cur_dp;
                                continue;
                            }
                            let mut ndiff = diff;
                            if dig % 2 == 0 {
                                ndiff += 1;
                            } else {
                                ndiff -= 1;
                            }
                            let nrem = (rem * 10 + dig) % k;
                            let mut nless = less;
                            if dig < digits[i] {
                                nless = 1;
                            }
                            dp[i + 1][ndiff][nrem][nless][0] += cur_dp;
                        }
                    }
                }
            }
        }
    }
    dp[len][len][0][1][0]
}

impl Solution {
    pub fn number_of_beautiful_integers(low: i32, high: i32, k: i32) -> i32 {
        let low = low as usize;
        let high = high as usize;
        let k = k as usize;

        let ans = solve(high + 1, k) - solve(low, k);
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
    #[case(10, 20, 3, 2)]
    #[case(1, 10, 1, 1)]
    #[case(5, 5, 2, 0)]
    fn case(#[case] low: i32, #[case] high: i32, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::number_of_beautiful_integers(low, high, k);
        assert_eq!(actual, expected);
    }
}
