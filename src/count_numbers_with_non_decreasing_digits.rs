//! Solution for https://leetcode.com/problems/count-numbers-with-non-decreasing-digits
//! 3519. Count Numbers with Non-Decreasing Digits

struct BigNum {
    d: Vec<i32>,
    base: i32,
}

impl BigNum {
    fn new(num: String, base: i32) -> Self {
        let mut d: Vec<_> = num.as_bytes().iter().map(|&c| (c - b'0') as i32).collect();
        d.reverse();
        Self { d, base }
    }

    fn get_remainder(&self, m: i32) -> i32 {
        let mut ans = 0;
        for i in (0..self.d.len()).rev() {
            ans = (ans * self.base + self.d[i]) % m;
        }
        ans
    }

    fn divide(&mut self, b: i32) {
        assert!(b > 0);
        let mut ans = Vec::new();
        let mut num = 0;
        for i in (0..self.d.len()).rev() {
            num = num * 10 + self.d[i];
            ans.push(num / b);
            num %= b;
        }
        ans.reverse();
        self.d = ans;
        self.norm();
    }

    fn add(&mut self, val: i32) {
        assert!(val >= 0);
        self.d[0] += val;
        self.norm();
    }

    fn norm(&mut self) {
        let mut carry = 0;
        for i in 0..self.d.len() {
            self.d[i] += carry;
            carry = self.d[i] / self.base;
            self.d[i] %= self.base;
        }
        while carry > 0 {
            self.d.push(carry % self.base);
            carry /= self.base;
        }
        while self.d.len() >= 2 && *self.d.last().unwrap() == 0 {
            self.d.pop();
        }
    }

    fn is_zero(&self) -> bool {
        assert!(!self.d.is_empty());
        self.d.len() == 1 && self.d[0] == 0
    }

    fn print(&self) {
        for i in (0..self.d.len()).rev() {
            eprint!("{} ", self.d[i]);
        }
        eprintln!()
    }

    fn convert_base(&mut self, b: i32) {
        let mut ans = Vec::new();
        while !self.is_zero() {
            let dig = self.get_remainder(b);
            ans.push(dig);
            self.divide(b);
        }
        self.d = ans;
        self.base = b;
    }
}

fn solve(num: &BigNum) -> Num {
    let b = num.base as usize;
    let n = num.d.len();
    // dp[last_dig][less]
    let mut dp = vec![vec![Num::new(0); 2]; b];
    dp[0][0] = Num::new(1);
    for i in 0..n {
        let mut ndp = vec![vec![Num::new(0); 2]; b];
        let num_dig = num.d[n - 1 - i] as usize;
        for last_dig in 0..b {
            for less in 0..2 {
                let cur_dp = dp[last_dig][less];
                if cur_dp.val == 0 {
                    continue;
                }
                for dig in last_dig..b {
                    if less == 0 && dig > num_dig {
                        continue;
                    }
                    let mut nless = less;
                    if dig < num_dig {
                        nless = 1;
                    }
                    ndp[dig][nless] += cur_dp;
                }
            }
        }
        dp = ndp;
    }
    let mut ans = Num::new(0);
    for dig in 0..b {
        ans += dp[dig][1];
    }
    ans
}

impl Solution {
    pub fn count_numbers(l: String, r: String, b: i32) -> i32 {
        let mut l = BigNum::new(l, 10);
        l.convert_base(b);
        let mut r = BigNum::new(r, 10);
        r.convert_base(b);
        r.add(1);

        let ans = solve(&r) - solve(&l);
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
    #[case("23", "28", 8, 3)]
    #[case("2", "7", 2, 2)]
    fn case(#[case] l: String, #[case] r: String, #[case] b: i32, #[case] expected: i32) {
        let actual = Solution::count_numbers(l, r, b);
        assert_eq!(actual, expected);
    }
}
