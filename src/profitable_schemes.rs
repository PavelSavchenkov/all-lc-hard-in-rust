//! Solution for https://leetcode.com/problems/profitable-schemes
//! 879. Profitable Schemes

impl Solution {
    pub fn profitable_schemes(
        n: i32,
        min_profit: i32,
        group: Vec<i32>,
        profit_from_crime: Vec<i32>,
    ) -> i32 {
        let n = n as usize;
        let n_crimes = group.len();
        let min_profit = min_profit as usize;

        let mut dp = vec![vec![vec![Num::new(0); min_profit + 1]; n + 1]; n_crimes + 1];
        dp[0][0][0] = Num::new(1);
        for crimes_considered in 0..n_crimes {
            for people_used in 0..=n {
                for profit in 0..=min_profit {
                    let cur_dp = dp[crimes_considered][people_used][profit];
                    if cur_dp.val == 0 {
                        continue;
                    }
                    // skip crime
                    dp[crimes_considered + 1][people_used][profit] += cur_dp;
                    // commit crime
                    let new_people_used = people_used + group[crimes_considered] as usize;
                    if new_people_used <= n {
                        let new_profit = (profit + profit_from_crime[crimes_considered] as usize)
                            .min(min_profit);
                        dp[crimes_considered + 1][new_people_used][new_profit] += cur_dp;
                    }
                }
            }
        }
        let mut ans = Num::new(0);
        for people in 0..=n {
            ans += dp[n_crimes][people][min_profit];
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

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, 3, vec![2,2], vec![2,3], 2)]
    #[case(10, 5, vec![2,3,5], vec![6,7,8], 7)]
    fn case(
        #[case] n: i32,
        #[case] min_profit: i32,
        #[case] group: Vec<i32>,
        #[case] profit: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::profitable_schemes(n, min_profit, group, profit);
        assert_eq!(actual, expected);
    }
}
