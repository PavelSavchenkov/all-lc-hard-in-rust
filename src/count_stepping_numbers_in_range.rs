//! Solution for https://leetcode.com/problems/count-stepping-numbers-in-range
//! 2801. Count Stepping Numbers in Range

fn solve(digits: &Vec<u8>) -> Num {
    let len = digits.len();
    let mut dp = vec![vec![vec![vec![Num::new(0); 2]; 2]; 10]; len + 1];
    dp[0][0][1][0] = Num::new(1);
    for i in 0..len {
        for prev in 0..10 {
            for is_all_zero in 0..2 {
                for is_less in 0..2 {
                    let cur_dp = dp[i][prev][is_all_zero][is_less];
                    if cur_dp.val == 0 {
                        continue;
                    }
                    if is_all_zero == 1 {
                        // continue zeros
                        assert!(prev == 0);
                        dp[i + 1][0][1][1] += cur_dp;
                    }
                    let next_candidates = if is_all_zero == 1 {
                        (1..10).collect::<Vec<usize>>()
                    } else {
                        let mut nxt = Vec::new();
                        if prev > 0 {
                            nxt.push(prev - 1);
                        }
                        if prev + 1 < 10 {
                            nxt.push(prev + 1);
                        }
                        nxt
                    };
                    for &next in &next_candidates {
                        if is_less == 0 && (next as u8) > digits[i] - b'0' {
                            continue;
                        }
                        let mut new_is_less = is_less;
                        if (next as u8) < digits[i] - b'0' {
                            new_is_less = 1;
                        }
                        dp[i + 1][next][0][new_is_less] += cur_dp;
                        // eprintln!(
                        //     "i + 1 = {}, next = {}, new_is_less = {}, is_less = {}",
                        //     i + 1,
                        //     next,
                        //     new_is_less,
                        //     is_less,
                        // );
                    }
                }
            }
        }
    }

    let mut ans = Num::new(0);
    for last in 0..10 {
        ans += dp[len][last][0][1];
    }
    ans
}

impl Solution {
    pub fn count_stepping_numbers(low: String, high: String) -> i32 {
        let high = to_u8(&high);
        let low = to_u8(&low);

        let mut ans = solve(&high);
        ans -= solve(&low);

        let mut is_high_ok = true;
        for i in 0..high.len() - 1 {
            if (high[i] as i32 - high[i + 1] as i32).abs() != 1 {
                is_high_ok = false;
                break;
            }
        }
        if is_high_ok {
            ans += Num::new(1);
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

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("1", "11", 10)]
    #[case("90", "101", 2)]
    fn case(#[case] low: String, #[case] high: String, #[case] expected: i32) {
        let actual = Solution::count_stepping_numbers(low, high);
        assert_eq!(actual, expected);
    }
}
