//! Solution for https://leetcode.com/problems/decode-ways-ii
//! 639. Decode Ways II

fn cnt_decode(v: &[u8], len: usize) -> u32 {
    match len {
        1 => {
            if v[0] == b'0' {
                return 0;
            }
            if v[0] == b'*' {
                return 9;
            }
            1
        }
        2 => {
            let mut ans = 0;
            for c in 10..=26 {
                let c0 = (c / 10) as u8 + b'0';
                let c1 = (c % 10) as u8 + b'0';
                let mut matches = true;
                let cv = vec![c0, c1];
                for i in 0..2 {
                    if v[i] != b'*' && v[i] != cv[i] {
                        matches = false;
                        break;
                    }
                    if v[i] == b'*' && cv[i] == b'0' {
                        matches = false;
                        break;
                    }
                }
                if matches {
                    ans += 1;
                }
            }
            ans
        }
        _ => panic!("Wrong len to decode"),
    }
}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = to_u8(&s);
        let n = s.len();

        let mut dp = vec![Num::new(0); n + 1];
        dp[0] = Num::new(1);
        for i in 0..n {
            let cur_dp = dp[i];
            for j in i..=(i + 1).min(n - 1) {
                let coef = cnt_decode(&s[i..j + 1], j - i + 1);
                dp[j + 1] += cur_dp * Num::new(coef);
            }
        }

        dp[n].val as i32
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
    #[case("*", 9)]
    #[case("1*", 18)]
    #[case("2*", 15)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::num_decodings(s);
        assert_eq!(actual, expected);
    }
}
