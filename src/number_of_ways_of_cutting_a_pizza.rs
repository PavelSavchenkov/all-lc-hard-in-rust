//! Solution for https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza
//! 1444. Number of Ways of Cutting a Pizza

impl Solution {
    pub fn ways(a: Vec<String>, k: i32) -> i32 {
        let a = to_u8_vec(&a);
        let k = k as usize;
        let n = a.len();
        let m = a[0].len();

        let mut right = vec![-1; n];
        for i in 0..n {
            for j in 0..m {
                if a[i][j] == b'A' {
                    right[i] = j as i32;
                }
            }
        }

        let mut bottom = vec![-1; m];
        for j in 0..m {
            for i in 0..n {
                if a[i][j] == b'A' {
                    bottom[j] = i as i32;
                }
            }
        }

        let mut dp = vec![vec![vec![Num::new(0); k]; m]; n];
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if right[i] >= j as i32 || (i + 1 < n && dp[i + 1][j][0].val == 1) {
                    dp[i][j][0] = Num::new(1);
                }
            }
        }
        for cuts in 1..k {
            for i in 0..n {
                for j in 0..m {
                    let mut ndp = Num::new(0);
                    // horizontal
                    let mut has_apple = false;
                    for i1 in i..n - 1 {
                        if right[i1] >= j as i32 {
                            has_apple = true;
                        }
                        if has_apple {
                            ndp += dp[i1 + 1][j][cuts - 1];
                        }
                    }
                    // vertical
                    has_apple = false;
                    for j1 in j..m - 1 {
                        if bottom[j1] >= i as i32 {
                            has_apple = true;
                        }
                        if has_apple {
                            ndp += dp[i][j1 + 1][cuts - 1];
                        }
                    }
                    dp[i][j][cuts] = ndp;
                }
            }
        }
        let ans = dp[0][0][k - 1];
        ans.val as i32
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
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
    #[case(vec!["A..".into(),"AAA".into(),"...".into()], 3, 3)]
    #[case(vec!["A..".into(),"AA.".into(),"...".into()], 3, 1)]
    #[case(vec!["A..".into(),"A..".into(),"...".into()], 1, 1)]
    fn case(#[case] pizza: Vec<String>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::ways(pizza, k);
        assert_eq!(actual, expected);
    }
}
