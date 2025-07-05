//! Solution for https://leetcode.com/problems/dice-roll-simulation
//! 1223. Dice Roll Simulation

impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let n = n as usize;
        let roll_max: Vec<_> = roll_max.iter().map(|&x| x as usize).collect();
        let max_consec = *roll_max.iter().max().unwrap();

        let mut dp = vec![vec![vec![Num::new(0); 6]; max_consec]; n + 1];
        dp[0][0][0] = Num::new(1);
        for i in 0..n {
            for consec in 0..max_consec {
                for last in 0..6 {
                    let cur_dp = dp[i][consec][last];
                    if cur_dp.val == 0 {
                        continue;
                    }
                    for next in 0..6 {
                        let mut new_consec = if next == last { consec + 1 } else { 0 };
                        if i == 0 {
                            new_consec = 0;
                        }
                        if next == last && new_consec + 1 > roll_max[last] {
                            continue;
                        }
                        dp[i + 1][new_consec][next] += cur_dp;
                    }
                }
            }
        }
        let mut ans = Num::new(0);
        for consec in 0..max_consec {
            for last in 0..6 {
                ans += dp[n][consec][last];
            }
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
    #[case(2, vec![1,1,2,2,2,3], 34)]
    #[case(2, vec![1,1,1,1,1,1], 30)]
    #[case(3, vec![1,1,1,2,2,3], 181)]
    fn case(#[case] n: i32, #[case] roll_max: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::die_simulator(n, roll_max);
        assert_eq!(actual, expected);
    }
}
