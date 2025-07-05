//! Solution for https://leetcode.com/problems/number-of-unique-good-subsequences
//! 1987. Number of Unique Good Subsequences

impl Solution {
    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        let s: Vec<usize> = binary
            .as_bytes()
            .iter()
            .map(|&b| if b == b'0' { 0 } else { 1 })
            .collect();
        let n = s.len();

        let mut dp = vec![Num::new(0); n];
        let mut pref = vec![Num::new(0); n + 1];

        let mut ans = Num::new(0);
        let mut last = vec![-1 as i32, -1];
        for i in 0..n {
            let prev = last[s[i]];
            last[s[i]] = i as i32;

            // continue
            dp[i] = pref[i] - pref[prev.max(0) as usize];

            // start new
            if s[i] == 1 && prev == -1 {
                dp[i] += Num::new(1);
            }

            // update prefix sums
            pref[i + 1] = pref[i] + dp[i];

            // update ans
            ans += dp[i];
        }

        for i in 0..n {
            if s[i] == 0 {
                ans += Num::new(1);
                break;
            }
        }

        ans.val as i32
    }
}

use std::cmp::Eq;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::vec;

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
    #[case("001", 2)]
    #[case("11", 2)]
    #[case("101", 5)]
    fn case(#[case] binary: String, #[case] expected: i32) {
        let actual = Solution::number_of_unique_good_subsequences(binary);
        assert_eq!(actual, expected);
    }
}
