//! Solution for https://leetcode.com/problems/count-the-number-of-winning-sequences
//! 3320. Count The Number of Winning Sequences

fn score(alice: usize, bob: usize) -> i32 {
    assert!(alice < 3);
    assert!(bob < 3);
    if alice == bob {
        return 0;
    }
    if (alice + 1) % 3 == bob {
        return 1;
    }
    assert!((bob + 1) % 3 == alice);
    -1
}

impl Solution {
    pub fn count_winning_sequences(s: String) -> i32 {
        let s = to_u8(&s);
        let n = s.len();

        let a: Vec<_> = s
            .iter()
            .map(|c| "FWE".as_bytes().iter().position(|b| b == c).unwrap())
            .collect();

        // dp[bob - alice][last_bob]
        let mut dp = vec![vec![Num::new(0); 3]; 2 * n + 1];
        for b in 0..3 {
            dp[(n as i32 + score(a[0], b)) as usize][b] += Num::new(1);
        }
        for i in 1..n {
            let mut ndp = vec![vec![Num::new(0); 3]; 2 * n + 1];
            for diff in 0..=2 * n {
                for last in 0..3 {
                    let cur_dp = dp[diff][last];
                    if cur_dp.val == 0 {
                        continue;
                    }
                    for cur in 0..3 {
                        if cur == last {
                            continue;
                        }
                        let ndiff = diff as i32 + score(a[i], cur);
                        ndp[ndiff as usize][cur] += cur_dp;
                    }
                }
            }
            dp = ndp;
        }
        let mut ans = Num::new(0);
        for diff in n + 1..=2 * n {
            for last in 0..3 {
                ans += dp[diff][last];
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
    #[case("FFF", 3)]
    #[case("FWEFW", 18)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::count_winning_sequences(s);
        assert_eq!(actual, expected);
    }
}
