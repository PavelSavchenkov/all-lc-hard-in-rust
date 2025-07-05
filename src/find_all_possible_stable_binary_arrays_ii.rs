//! Solution for https://leetcode.com/problems/find-all-possible-stable-binary-arrays-ii
//! 3130. Find All Possible Stable Binary Arrays II

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let zero = zero as usize;
        let one = one as usize;
        let limit = limit as usize;

        let mut dp = vec![vec![vec![Num::new(0); 2]; one + 1]; zero + 1];
        dp[1][0][0] = Num::new(1);
        dp[0][1][1] = Num::new(1);
        for total in 2..=zero + one {
            for z in 0..=zero.min(total) {
                let o = total - z;
                if o > one {
                    continue;
                }

                // last is 1
                if o > 0 {
                    let mut d = dp[z][o - 1][0] + dp[z][o - 1][1];
                    if o >= limit + 1 {
                        if limit + 1 == total {
                            d -= Num::new(1);
                        } else {
                            d -= dp[z][o - (limit + 1)][0];
                        }
                    }
                    dp[z][o][1] = d;
                }
                // last is 0
                if z > 0 {
                    let mut d = dp[z - 1][o][0] + dp[z - 1][o][1];
                    if z >= limit + 1 {
                        if limit + 1 == total {
                            d -= Num::new(1);
                        } else {
                            d -= dp[z - (limit + 1)][o][1];
                        }
                    }
                    dp[z][o][0] = d;
                }
            }
        }

        let ans = dp[zero][one][0] + dp[zero][one][1];
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
    #[case(1, 1, 2, 2)]
    #[case(1, 2, 1, 1)]
    #[case(3, 3, 2, 14)]
    fn case(#[case] zero: i32, #[case] one: i32, #[case] limit: i32, #[case] expected: i32) {
        let actual = Solution::number_of_stable_arrays(zero, one, limit);
        assert_eq!(actual, expected);
    }
}
