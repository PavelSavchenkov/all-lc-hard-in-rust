//! Solution for https://leetcode.com/problems/count-different-palindromic-subsequences
//! 730. Count Different Palindromic Subsequences

const A: usize = 4;

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let s = to_u8(&s);
        let s: Vec<_> = s.iter().map(|&c| (c - b'a') as usize).collect();
        let n = s.len();

        let mut next = vec![vec![n; A]; n];
        for i in (0..n).rev() {
            if i + 1 < n {
                for c in 0..A {
                    next[i][c] = next[i + 1][c];
                }
            }
            next[i][s[i]] = i;
        }

        let mut prev = vec![vec![-1; A]; n];
        for i in 0..n {
            if i > 0 {
                for c in 0..A {
                    prev[i][c] = prev[i - 1][c];
                }
            }
            prev[i][s[i]] = i as i32;
        }

        let mut dp = vec![vec![Num::new(0); n]; n];
        for len in 1..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                let mut ndp = Num::new(0);
                for c in 0..A {
                    let lc = next[l][c];
                    let rc = prev[r][c];
                    if rc == -1 {
                        continue;
                    }
                    let rc = rc as usize;
                    if lc > rc {
                        continue;
                    }
                    if lc + 2 <= rc {
                        ndp += dp[lc + 1][rc - 1];
                    }
                    ndp += Num::new(1);
                    if lc + 1 <= rc {
                        ndp += Num::new(1);
                    }
                }
                dp[l][r] = ndp;
            }
        }

        dp[0][n - 1].val as i32
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
    #[case("bccb", 6)]
    #[case(
        "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba",
        104860361
    )]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::count_palindromic_subsequences(s);
        assert_eq!(actual, expected);
    }
}
