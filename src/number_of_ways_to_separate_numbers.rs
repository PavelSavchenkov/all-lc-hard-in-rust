//! Solution for https://leetcode.com/problems/number-of-ways-to-separate-numbers
//! 1977. Number of Ways to Separate Numbers

use std::cmp::Eq;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::vec;

impl Solution {
    pub fn number_of_combinations(num: String) -> i32 {
        let s = num.as_bytes();
        let n = num.len();

        let mut lcp = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if s[i] == s[j] {
                    lcp[i][j] = 1;
                    if i + 1 < n && j + 1 < n {
                        lcp[i][j] += lcp[i + 1][j + 1];
                    }
                }
            }
        }

        // dp[i][len] -- s[i] is the first not considered, last num had "len" length
        let mut dp = vec![vec![Num::new(0); n + 1]; n + 1];
        // dp[i][<len]
        let mut dp_sum = vec![vec![Num::new(0); n + 2]; n + 1];

        for i in 0..=n {
            for len in 1..=i {
                if s[i - len] == b'0' {
                    continue;
                }
                let mut cur = dp_sum[i - len][len];
                if i >= 2 * len {
                    let j1 = i - 2 * len;
                    let j2 = i - len;
                    if s[j1] != b'0' {
                        let pref = lcp[j1][j2];
                        // eprintln!("j1={j1}, j2={j2}, pref={pref}");
                        if pref >= len || s[j1 + pref] < s[j2 + pref] {
                            cur += dp[j2][len];
                        }
                    }
                }
                dp[i][len] = cur;
            }
            if i == 0 {
                dp[0][0] = Num::new(1);
            }

            dp_sum[i][0] = Num::new(0);
            for len in 1..=(n + 1) {
                dp_sum[i][len] = dp_sum[i][len - 1] + dp[i][len - 1];
            }
        }

        // eprintln!("s={:#?}", num);
        // for i in 0..=n {
        //     for len in 0..=n {
        //         eprintln!("i={i}, len={len}, dp={}", dp[i][len].val);
        //     }
        //     for len in 0..=n + 1 {
        //         eprintln!("i={i}, len={len}, dp_sum={}", dp_sum[i][len].val);
        //     }
        // }

        dp_sum[n][n + 1].val as i32
    }
}

type Num = NumMod<1_000_000_007>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NumMod<const M: u32> {
    val: u32,
}

impl<const M: u32> NumMod<M> {
    // doesn't work
    const static_assert: () = {
        assert!(M < (1 << 31));
    };

    fn new(x: u32) -> Self {
        NumMod { val: x % M }
    }

    // x and y are already normalized
    fn safe_add_mod(mut x: u32, y: u32) -> u32 {
        x += y;
        if x >= M {
            x -= M;
        }
        x
    }

    fn safe_sub_mod(mut x: u32, y: u32) -> u32 {
        x += M - y;
        if x >= M {
            x -= M;
        }
        x
    }

    fn safe_mul_mod(x: u32, y: u32) -> u32 {
        ((x as u64 * y as u64) % M as u64) as u32
    }
}

impl<const M: u32> Add for NumMod<M> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            val: Self::safe_add_mod(self.val, other.val),
        }
    }
}

impl<const M: u32> AddAssign for NumMod<M> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<const M: u32> Sub for NumMod<M> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            val: Self::safe_sub_mod(self.val, other.val),
        }
    }
}

impl<const M: u32> SubAssign for NumMod<M> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<const M: u32> Mul for NumMod<M> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            val: Self::safe_mul_mod(self.val, other.val),
        }
    }
}

impl<const M: u32> MulAssign for NumMod<M> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("327", 2)]
    #[case("094", 0)]
    #[case("0", 0)]
    #[case("11", 2)]
    fn case(#[case] num: String, #[case] expected: i32) {
        let actual = Solution::number_of_combinations(num);
        assert_eq!(actual, expected);
    }
}
