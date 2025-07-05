//! Solution for https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid
//! 1411. Number of Ways to Paint N × 3 Grid

fn mask_to_vec(mask: usize) -> Vec<usize> {
    vec![mask / 9, mask / 3 % 3, mask % 3]
}

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let n = n as usize;

        let mut good_masks = Vec::new();
        for mask in 0..27 {
            let row = mask_to_vec(mask);
            if row[0] != row[1] && row[1] != row[2] {
                good_masks.push(mask);
            }
        }

        let mut good_next_masks = vec![Vec::new(); 27];
        for &m1 in &good_masks {
            for &m2 in &good_masks {
                let r1 = mask_to_vec(m1);
                let r2 = mask_to_vec(m2);
                let mut is_good = true;
                for j in 0..3 {
                    if r1[j] == r2[j] {
                        is_good = false;
                        break;
                    }
                }
                if is_good {
                    good_next_masks[m1].push(m2);
                }
            }
        }

        let mut dp = vec![Num::new(0); 27];
        for &mask in &good_masks {
            dp[mask] = Num::new(1);
        }
        for i in 1..n {
            let mut ndp = vec![Num::new(0); 27];
            for &prev_mask in &good_masks {
                for &mask in &good_next_masks[prev_mask] {
                    ndp[mask] += dp[prev_mask];
                }
            }
            dp = ndp;
        }

        let ans = dp.iter().fold(Num::new(0), |acc, e| acc + *e);
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
    #[case(1, 12)]
    #[case(5000, 30228214)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::num_of_ways(n);
        assert_eq!(actual, expected);
    }
}
