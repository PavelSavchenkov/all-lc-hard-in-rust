//! Solution for https://leetcode.com/problems/count-of-integers
//! 2719. Count of Integers

fn solve(mut R: u128, min_sum: usize, max_sum: usize) -> Num {
    let mut digits = Vec::new();
    while R > 0 {
        digits.push((R % 10) as usize);
        R /= 10;
    }
    digits.reverse();

    let mut dp = vec![vec![vec![Num::new(0); 2]; max_sum + 1]; digits.len() + 1];
    dp[0][0][0] = Num::new(1);
    for i in 0..digits.len() {
        for sum in 0..=max_sum {
            for less in 0..2 {
                let cur_dp = dp[i][sum][less];
                if cur_dp.val == 0 {
                    continue;
                }
                for digit in 0..=9 {
                    if digit > digits[i] && less == 0 {
                        continue;
                    }
                    if sum + digit > max_sum {
                        continue;
                    }
                    let nless = less == 1 || digit < digits[i];
                    dp[i + 1][sum + digit][nless as usize] += cur_dp;
                }
            }
        }
    }

    let mut ans = Num::new(0);
    for s in min_sum..=max_sum {
        ans += dp[digits.len()][s][1];
    }
    ans
}

impl Solution {
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let L: u128 = num1.parse().expect("Valid number");
        let R: u128 = num2.parse().expect("Valid number");
        let min_sum = min_sum as usize;
        let max_sum = max_sum as usize;

        let mut ans = solve(R + 1, min_sum, max_sum);
        ans -= solve(L, min_sum, max_sum);
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

    fn pow_mod(mut x: u32, mut p: u32) -> u32 {
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
        Self::pow_mod(x, MOD - 2)
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
    #[case("1", "12", 1, 8, 11)]
    #[case("1", "5", 1, 5, 5)]
    fn case(
        #[case] num1: String,
        #[case] num2: String,
        #[case] min_sum: i32,
        #[case] max_sum: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::count(num1, num2, min_sum, max_sum);
        assert_eq!(actual, expected);
    }
}
