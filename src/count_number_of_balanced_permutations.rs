//! Solution for https://leetcode.com/problems/count-number-of-balanced-permutations
//! 3343. Count Number of Balanced Permutations

impl Solution {
    pub fn count_balanced_permutations(num: String) -> i32 {
        let num = to_u8(&num);

        let mut cnt = vec![0; 10];
        let mut sum_all = 0;
        for ch in &num {
            let d = (ch - b'0') as usize;
            sum_all += d;
            cnt[d] += 1;
        }

        if sum_all % 2 != 0 {
            return 0;
        }
        let sum = sum_all / 2;
        let n1 = num.len() / 2;
        let n2 = num.len() - n1;

        let mut binom = vec![vec![Num::new(0); num.len() + 1]; num.len() + 1];
        binom[0][0] = Num::new(1);
        for i in 1..binom.len() {
            binom[i][0] = Num::new(1);
            for j in 1..=i {
                binom[i][j] = binom[i - 1][j] + binom[i - 1][j - 1];
            }
        }

        let mut dp = vec![vec![vec![Num::new(0); sum + 1]; n2 + 1]; n1 + 1];
        dp[0][0][0] = Num::new(1);
        for d in 0..10 {
            for have1 in (0..=n1).rev() {
                for have2 in (0..=n2).rev() {
                    for s1 in 0..=sum {
                        let cur_dp = dp[have1][have2][s1];
                        if cur_dp.val == 0 || cnt[d] == 0 {
                            continue;
                        }
                        for cnt1 in 0..=cnt[d] {
                            let nhave1 = have1 + cnt1;
                            if nhave1 > n1 {
                                break;
                            }
                            let cnt2 = cnt[d] - cnt1;
                            let nhave2 = have2 + cnt2;
                            if nhave2 > n2 {
                                continue;
                            }
                            let ns1 = s1 + cnt1 * d;
                            if ns1 > sum {
                                break;
                            }
                            dp[nhave1][nhave2][ns1] +=
                                cur_dp * binom[nhave1][cnt1] * binom[nhave2][cnt2];
                        }
                    }
                }
            }
        }
        let ans = dp[n1][n2][sum];
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
use std::vec;

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
    #[case("123", 2)]
    #[case("112", 1)]
    #[case("12345", 0)]
    fn case(#[case] num: String, #[case] expected: i32) {
        let actual = Solution::count_balanced_permutations(num);
        assert_eq!(actual, expected);
    }
}
