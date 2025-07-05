//! Solution for https://leetcode.com/problems/find-the-sum-of-subsequence-powers
//! 3098. Find the Sum of Subsequence Powers

impl Solution {
    pub fn sum_of_powers(mut a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let k = k as usize;

        a.sort();

        let mut diffs = Vec::new();
        for i in 0..n {
            for j in 0..i {
                diffs.push(a[i] - a[j]);
            }
        }
        diffs.sort();
        diffs.dedup();
        diffs.reverse();

        let mut ans = Num::new(0);
        let mut prev = Num::new(0);
        for &diff in &diffs {
            let mut dp = vec![vec![Num::new(0); k + 1]; n + 1];
            for i in 0..n {
                for cnt in 1..=k {
                    if cnt == 1 {
                        dp[i][cnt] = Num::new(1);
                        continue;
                    }
                    for j in 0..i {
                        if a[i] - a[j] < diff {
                            break;
                        }
                        let prev_dp = dp[j][cnt - 1];
                        dp[i][cnt] += prev_dp;
                    }
                }
            }
            let mut cur = Num::new(0);
            for i in 0..n {
                cur += dp[i][k];
            }
            ans += (cur - prev) * Num::new(diff as u32);
            prev = cur;
        }
        ans.val as i32
    }
}

use std::cmp::Eq;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

const MOD: u32 = 1_000_000_007;
type Num = NumMod<MOD>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

    fn safe_div_mod(x: u32, y: u32) -> u32 {
        Self::safe_mul_mod(x, Self::inv_mod(y))
    }

    fn pow(&self, p: u64) -> NumMod<M> {
        NumMod::new(Self::pow_mod(self.val, p))
    }

    fn pow_mod(mut x: u32, mut p: u64) -> u32 {
        let mut ans = 1;
        while p > 0 {
            if p % 2 == 1 {
                ans = Self::safe_mul_mod(ans, x);
            }
            x = Self::safe_mul_mod(x, x);
            p /= 2;
        }
        ans
    }

    fn inv_mod(x: u32) -> u32 {
        assert!(x != 0);
        Self::pow_mod(x, (MOD - 2) as u64)
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

impl<const M: u32> Div for NumMod<M> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            val: Self::safe_div_mod(self.val, other.val),
        }
    }
}

impl<const M: u32> DivAssign for NumMod<M> {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4], 3, 4)]
    #[case(vec![2,2], 2, 0)]
    #[case(vec![4,3,-1], 2, 10)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::sum_of_powers(nums, k);
        assert_eq!(actual, expected);
    }
}
