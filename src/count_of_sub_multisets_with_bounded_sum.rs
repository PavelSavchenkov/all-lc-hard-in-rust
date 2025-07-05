//! Solution for https://leetcode.com/problems/count-of-sub-multisets-with-bounded-sum
//! 2902. Count of Sub-Multisets With Bounded Sum

impl Solution {
    pub fn count_sub_multisets(a: Vec<i32>, l: i32, r: i32) -> i32 {
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let n = a.len();
        let l = l as usize;
        let r = r as usize;
        let M = r;

        let mut cnt = vec![0; M + 1];
        let mut vals = Vec::new();
        for &x in &a {
            if x <= M {
                cnt[x] += 1;
                vals.push(x);
            }
        }

        let cnt_zeros = cnt[0];
        vals.sort();
        vals.dedup();
        // about sqrt(r) values
        if vals[0] == 0 {
            vals.remove(0);
        }

        let mut dp = vec![Num::new(0); M + 1];
        dp[0] = Num::new(1);
        for &val in &vals {
            let occ = cnt[val];
            assert!(occ > 0);
            let mut ndp = dp.clone();
            for rem in 0..val {
                let mut sum_dp = Num::new(0);
                for sum in (rem..=M).step_by(val) {
                    ndp[sum] += sum_dp;
                    if sum >= val * occ {
                        sum_dp -= dp[sum - val * occ];
                    }
                    sum_dp += dp[sum];
                }
            }
            dp = ndp;
        }

        let coef = Num::new((cnt_zeros + 1) as u32);
        let mut ans = Num::new(0);
        for sum in l..=r {
            ans += dp[sum];
        }
        ans *= coef;
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
    #[case(vec![1,2,2,3], 6, 6, 1)]
    #[case(vec![2,1,4,2,7], 1, 5, 7)]
    #[case(vec![1,2,1,3,5,2], 3, 5, 9)]
    fn case(#[case] nums: Vec<i32>, #[case] l: i32, #[case] r: i32, #[case] expected: i32) {
        let actual = Solution::count_sub_multisets(nums, l, r);
        assert_eq!(actual, expected);
    }
}
