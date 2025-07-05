//! Solution for https://leetcode.com/problems/count-the-number-of-infection-sequences
//! 2954. Count the Number of Infection Sequences

impl Solution {
    pub fn number_of_sequence(n: i32, sick: Vec<i32>) -> i32 {
        // let n = 100000;
        // let mut sick = Vec::new();
        // for i in (0..n).step_by(2) {
        //     sick.push(i as i32);
        // }

        let n = n as usize;
        let sick: Vec<_> = sick.iter().map(|&x| x as usize).collect();

        let mut inv = vec![Num::new(1); n + 1];
        for i in 2..=n {
            let x1 = inv[MOD as usize % i];
            let coef = Num::new(MOD - MOD / i as u32);
            inv[i] = coef * x1;
            assert!(inv[i] * Num::new(i as u32) == Num::new(1));
        }

        let mut fact = vec![Num::new(1); n + 1];
        let mut inv_fact = vec![Num::new(1); n + 1];
        let mut pow2 = vec![Num::new(1); n + 1];
        for i in 1..=n {
            fact[i] = fact[i - 1] * Num::new(i as u32);
            // inv_fact[i] = Num::new(1) / fact[i];
            inv_fact[i] = inv_fact[i - 1] * inv[i];
            pow2[i] = pow2[i - 1] * Num::new(2);
        }

        let mut ans = Num::new(1);
        assert!(!sick.is_empty());
        for i in 0..sick.len() - 1 {
            let len = sick[i + 1] - sick[i] - 1;
            if len == 0 {
                continue;
            }
            ans *= pow2[len - 1];
            ans *= inv_fact[len];
        }
        ans *= inv_fact[sick[0]];
        ans *= inv_fact[n - 1 - sick.last().unwrap()];
        ans *= fact[n - sick.len()];
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
    #[case(5, vec![0,4], 4)]
    #[case(4, vec![1], 3)]
    fn case(#[case] n: i32, #[case] sick: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::number_of_sequence(n, sick);
        assert_eq!(actual, expected);
    }
}
