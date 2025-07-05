//! Solution for https://leetcode.com/problems/find-sum-of-array-product-of-magical-sequences
//! 3539. Find Sum of Array Product of Magical Sequences

impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        let m = m as usize;
        let k = k as usize;
        let n = nums.len();
        let MAX_SUFF = m;

        let mut binom = vec![vec![Num::new(0); m + 1]; m + 1];
        binom[0][0] = Num::new(1);
        for i in 1..binom.len() {
            binom[i][0] = Num::new(1);
            for j in 1..=i {
                binom[i][j] = binom[i - 1][j - 1] + binom[i - 1][j];
            }
        }

        let mut dp = vec![vec![vec![Num::new(0); MAX_SUFF + 1]; k + 1]; m + 1];
        dp[0][0][0] = Num::new(1);
        for x in 0..n {
            let mut ndp = vec![vec![vec![Num::new(0); MAX_SUFF + 1]; k + 1]; m + 1];
            for covered in 0..=m {
                for ones in 0..=k {
                    for suff in 0..=MAX_SUFF {
                        let cur_dp = dp[covered][ones][suff];
                        if cur_dp.val == 0 {
                            continue;
                        }
                        let mut coef_nums = Num::new(1);
                        for cnt_x in 0..=m - covered {
                            if cnt_x == 0 {
                                let nones = ones + suff % 2;
                                if nones > k {
                                    continue;
                                }
                                ndp[covered][nones][suff / 2] += cur_dp;
                                continue;
                            }
                            coef_nums *= Num::new(nums[x] as u32);
                            let coef = binom[covered + cnt_x][cnt_x];
                            let nsuff = (suff + cnt_x) / 2;
                            assert!(nsuff <= MAX_SUFF);
                            let nones = ones + (suff + cnt_x) % 2;
                            if nones > k {
                                continue;
                            }
                            ndp[covered + cnt_x][nones][nsuff] += cur_dp * coef * coef_nums;
                        }
                    }
                }
            }
            dp = ndp;
        }
        let mut ans = Num::new(0);
        for ones in 0..=k {
            for suff in 0..=MAX_SUFF {
                if ones + suff.count_ones() as usize == k {
                    ans += dp[m][ones][suff];
                }
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
    #[case(5, 5, vec![1,10,100,10000,1000000], 991600007)]
    #[case(2, 2, vec![5,4,3,2,1], 170)]
    #[case(1, 1, vec![28], 28)]
    fn case(#[case] m: i32, #[case] k: i32, #[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::magical_sum(m, k, nums);
        assert_eq!(actual, expected);
    }
}
