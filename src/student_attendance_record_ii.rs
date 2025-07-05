//! Solution for https://leetcode.com/problems/student-attendance-record-ii
//! 552. Student Attendance Record II

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let n = n as usize;

        let mut dp = vec![vec![vec![Num::new(0); 3]; 2]; n + 1];
        dp[0][0][0] = Num::new(1);
        for i in 0..n {
            for absent in 0..2 {
                for L in 0..3 {
                    let cur_dp = dp[i][absent][L];
                    if cur_dp.val == 0 {
                        continue;
                    }

                    // P
                    dp[i + 1][absent][0] += cur_dp;

                    // A
                    if absent + 1 < 2 {
                        dp[i + 1][absent + 1][0] += cur_dp;
                    }

                    // L
                    if L + 1 < 3 {
                        dp[i + 1][absent][L + 1] += cur_dp;
                    }
                }
            }
        }

        let mut ans = Num::new(0);
        for absent in 0..2 {
            for L in 0..3 {
                ans += dp[n][absent][L];
            }
        }

        ans.val as i32
    }
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
    #[case(2, 8)]
    #[case(1, 3)]
    #[case(10101, 183236316)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::check_record(n);
        assert_eq!(actual, expected);
    }
}
