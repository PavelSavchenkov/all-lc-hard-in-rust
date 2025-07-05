//! Solution for https://leetcode.com/problems/maximize-number-of-nice-divisors
//! 1808. Maximize Number of Nice Divisors

fn get_sqrt(n: i32) -> i32 {
    let mut s = (n as f64).sqrt().round() as i32;
    while s * s > n {
        s -= 1;
    }
    while s * s < n {
        s += 1;
    }
    s
}

fn print_up_to(n: usize) {
    let mut dp = vec![0 as u64; n + 1];
    let mut optim_last = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        for last in 1..=i {
            let cand = dp[i - last] * last as u64;
            if cand >= dp[i] {
                dp[i] = cand;
                optim_last[i] = last;
            }
        }
    }

    for i in 1..=n {
        eprint!("{:3} = ", i);
        let mut j = i;
        while j > 0 {
            let last = optim_last[j];
            eprint!(" {}", last);
            j -= last;
        }
        eprintln!("; sqrt = {}, mult = {}", get_sqrt(i as i32), dp[i]);
    }
}

impl Solution {
    pub fn max_nice_divisors(prime_factors: i32) -> i32 {
        // print_up_to(50);
        // return 0;

        if prime_factors == 1 {
            return 1;
        }

        let ans;
        match prime_factors % 3 {
            0 => {
                ans = Num::new(3).pow((prime_factors / 3) as u64);
            }
            1 => {
                let cnt = (prime_factors / 3) as u64;
                ans = Num::new(3).pow(cnt - 1) * Num::new(4);
            }
            2 => {
                let cnt = ((prime_factors + 2) / 3) as u64;
                ans = Num::new(3).pow(cnt - 1) * Num::new(2);
            }
            _ => panic!(),
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
    #[case(5, 6)]
    #[case(8, 18)]
    #[case(12, 81)]
    fn case(#[case] prime_factors: i32, #[case] expected: i32) {
        let actual = Solution::max_nice_divisors(prime_factors);
        assert_eq!(actual, expected);
    }
}
