//! Solution for https://leetcode.com/problems/count-palindromic-subsequences
//! 2484. Count Palindromic Subsequences

impl Solution {
    pub fn count_palindromes(s: String) -> i32 {
        let s = to_u8(&s);
        let n = s.len();

        let mut ans = Num::new(0);
        for half in 0..1000 {
            let mut t = vec![0; 5];
            let mut tmp = half;
            for it in 0..3 {
                t[2 - it] = (tmp % 10) as u8 + b'0';
                t[2 + it] = t[2 - it];
                tmp /= 10;
            }
            let m = t.len();

            let mut dp = vec![vec![Num::new(0); m + 1]; n + 1];
            for i in 0..=n {
                for j in 0..=m {
                    if j == 0 {
                        dp[i][j] = Num::new(1);
                        continue;
                    }
                    let mut ndp = dp[i][j];
                    if i > 0 {
                        ndp += dp[i - 1][j];
                    }
                    if i > 0 && j > 0 && s[i - 1] == t[j - 1] {
                        ndp += dp[i - 1][j - 1];
                    }
                    dp[i][j] = ndp;
                }
            }
            ans += dp[n][m];
        }
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
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

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
    #[case("103301", 2)]
    #[case("0000000", 21)]
    #[case("9999900000", 2)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::count_palindromes(s);
        assert_eq!(actual, expected);
    }
}
