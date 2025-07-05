//! Solution for https://leetcode.com/problems/find-all-good-strings
//! 1397. Find All Good Strings

fn calc_prefix_function(s: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let mut p = vec![0; n];
    for i in 1..n {
        let mut j = p[i - 1];
        while j > 0 && s[i] != s[j] {
            j = p[j - 1];
        }
        if s[i] == s[j] {
            p[i] = j + 1;
        }
    }
    p
}

const A: usize = 26;

impl Solution {
    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        let n = n as usize;
        let s1 = to_u8(&s1);
        let s2 = to_u8(&s2);
        let evil = to_u8(&evil);

        let p = calc_prefix_function(&evil);
        let mut go = vec![vec![0; A]; evil.len()];
        for pref in 0..evil.len() {
            for c in 0..A {
                let ch = b'a' + c as u8;
                if evil[pref] == ch {
                    go[pref][c] = pref + 1;
                } else if pref != 0 {
                    go[pref][c] = go[p[pref - 1]][c];
                }
            }
        }

        let mut ans = Num::new(0);

        // i, eq1, eq2, pref
        let mut dp = vec![vec![vec![vec![Num::new(0); evil.len()]; 2]; 2]; n + 1];
        dp[0][1][1][0] = Num::new(1);
        for i in 0..=n {
            for eq1 in 0..2 {
                for eq2 in 0..2 {
                    for pref in 0..evil.len() {
                        let cur_dp = dp[i][eq1][eq2][pref];
                        if cur_dp.val == 0 {
                            continue;
                        }
                        if i == n {
                            ans += cur_dp;
                            continue;
                        }
                        for c in 0..A {
                            let ch = b'a' + c as u8;
                            if eq1 == 1 && ch < s1[i] {
                                continue;
                            }
                            if eq2 == 1 && s2[i] < ch {
                                continue;
                            }
                            let npref = go[pref][c];
                            if npref == evil.len() {
                                continue;
                            }
                            let mut neq1 = eq1;
                            if ch != s1[i] {
                                neq1 = 0;
                            }
                            let mut neq2 = eq2;
                            if ch != s2[i] {
                                neq2 = 0;
                            }
                            dp[i + 1][neq1][neq2][npref] += cur_dp;
                        }
                    }
                }
            }
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
    #[case(2, "aa", "da", "b", 51)]
    #[case(8, "leetcode", "leetgoes", "leet", 0)]
    #[case(2, "gx", "gz", "x", 2)]
    fn case(
        #[case] n: i32,
        #[case] s1: String,
        #[case] s2: String,
        #[case] evil: String,
        #[case] expected: i32,
    ) {
        let actual = Solution::find_good_strings(n, s1, s2, evil);
        assert_eq!(actual, expected);
    }
}
