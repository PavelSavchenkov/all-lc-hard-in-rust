//! Solution for https://leetcode.com/problems/count-vowels-permutation
//! 1220. Count Vowels Permutation

fn to_ch(c: usize) -> u8 {
    match c {
        0 => b'a',
        1 => b'e',
        2 => b'i',
        3 => b'o',
        4 => b'u',
        _ => panic!("Wrong c = {}", c),
    }
}

fn ok(ch: u8, ch2: u8) -> bool {
    match ch {
        b'a' => ch2 == b'e',
        b'e' => ch2 == b'a' || ch2 == b'i',
        b'i' => ch2 != b'i',
        b'o' => ch2 == b'i' || ch2 == b'u',
        b'u' => ch2 == b'a',
        _ => panic!("Wrong ch = {}", ch as char),
    }
}

const A: usize = 5;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let n = n as usize;

        let mut dp = vec![Num::new(1); A];
        for i in 1..n {
            let mut ndp = vec![Num::new(0); A];
            for c in 0..A {
                let cur_dp = dp[c];
                for c2 in 0..A {
                    if ok(to_ch(c), to_ch(c2)) {
                        ndp[c2] += cur_dp;
                    }
                }
            }
            dp = ndp;
        }

        let ans = dp.iter().fold(Num::new(0), |acc, &e| acc + e);
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
    #[case(1, todo!("todo!(\"Failed to get solutions\""))]
    #[case(2, todo!("todo!(\"Failed to get solutions\""))]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::count_vowel_permutation(n);
        assert_eq!(actual, expected);
    }
}
