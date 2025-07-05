//! Solution for https://leetcode.com/problems/find-the-original-typed-string-ii
//! 3333. Find the Original Typed String II

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        let s = to_u8(&word);
        let k = k as usize;
        let n = s.len();

        let mut cnt = Vec::new();
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && s[i] == s[j] {
                j += 1;
            }
            cnt.push(j - i);
            i = j;
        }

        let mut ans = Num::new(1);
        for &c in &cnt {
            ans *= Num::new(c as u32);
        }

        if cnt.len() < k {
            let sum = k - cnt.len() - 1;
            let mut dp = vec![Num::new(0); sum + 1];
            let mut pref_dp = vec![Num::new(0); dp.len() + 1];
            dp[0] = Num::new(1);
            for &c in &cnt {
                assert!(c > 0);
                let c = c - 1;

                pref_dp[0] = Num::new(0);
                for i in 0..dp.len() {
                    pref_dp[i + 1] = pref_dp[i] + dp[i];
                }

                for s in (0..=sum).rev() {
                    // ndp[s] = dp[s] + dp[s - 1] + ... + dp[s - c]
                    let l = if s >= c { s - c } else { 0 };
                    dp[s] = pref_dp[s + 1] - pref_dp[l];
                }
            }
            let mut bad = Num::new(0);
            for s in 0..=sum {
                bad += dp[s];
            }
            ans -= bad;
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
    #[case("aabbccdd", 7, 5)]
    #[case("aabbccdd", 8, 1)]
    #[case("aaabbb", 3, 8)]
    fn case(#[case] word: String, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::possible_string_count(word, k);
        assert_eq!(actual, expected);
    }
}
