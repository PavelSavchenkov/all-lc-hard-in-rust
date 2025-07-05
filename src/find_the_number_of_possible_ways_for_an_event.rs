//! Solution for https://leetcode.com/problems/find-the-number-of-possible-ways-for-an-event
//! 3317. Find the Number of Possible Ways for an Event

impl Solution {
    pub fn number_of_ways(n: i32, x: i32, y: i32) -> i32 {
        let n = n as usize;
        let x = x as usize;
        let y = y as usize;

        let mut binom = vec![vec![Num::new(0); x + 1]; x + 1];
        binom[0][0] = Num::new(1);
        for i in 1..=x {
            binom[i][0] = Num::new(1);
            for j in 1..=i {
                binom[i][j] = binom[i - 1][j - 1] + binom[i - 1][j];
            }
        }

        // ways[b] -- the number of ways to distribute n performers into b groups
        // so each group has at least one performer
        let mut ways = vec![Num::new(0); x + 1];
        for b in 1..=x {
            let mut cur = Num::new(b as u32).pow(n as u64);
            for bb in 1..b {
                cur -= ways[bb] * binom[b][bb];
            }
            ways[b] = cur;
        }

        let y = Num::new(y as u32);
        let mut ans = Num::new(0);
        for b in 1..=x {
            let mut cur = ways[b];
            cur *= binom[x][b];
            cur *= y.pow(b as u64);
            ans += cur;
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
    #[case(1, 2, 3, 6)]
    #[case(5, 2, 1, 32)]
    #[case(3, 3, 4, 684)]
    fn case(#[case] n: i32, #[case] x: i32, #[case] y: i32, #[case] expected: i32) {
        let actual = Solution::number_of_ways(n, x, y);
        assert_eq!(actual, expected);
    }
}
