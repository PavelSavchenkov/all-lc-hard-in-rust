//! Solution for https://leetcode.com/problems/count-the-number-of-inversions
//! 3193. Count the Number of Inversions

struct Req {
    end: usize,
    cnt: usize,
}

impl Solution {
    pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut reqs: Vec<_> = requirements
            .iter()
            .map(|r| Req {
                end: r[0] as usize,
                cnt: r[1] as usize,
            })
            .collect();
        reqs.sort_by_key(|r| r.end);
        let max_cnt = reqs.iter().fold(0, |acc, e| acc.max(e.cnt));

        let mut ptr = 0;
        let mut dp = vec![Num::new(0); max_cnt + 1];
        dp[0] = Num::new(1);
        for i in 0..n {
            let mut ndp = vec![Num::new(0); max_cnt + 1];
            for cnt in 0..=max_cnt {
                let cur_dp = dp[cnt];
                if cur_dp.val == 0 {
                    continue;
                }
                for new in 0..=i {
                    let ncnt = cnt + new;
                    if ncnt > max_cnt {
                        continue;
                    }
                    ndp[ncnt] += cur_dp;
                }
            }

            dp = ndp;

            while ptr + 1 < reqs.len() && reqs[ptr + 1].end <= i {
                ptr += 1;
            }
            if ptr < reqs.len() && reqs[ptr].end == i {
                for cnt in 0..=max_cnt {
                    if cnt != reqs[ptr].cnt {
                        dp[cnt] = Num::new(0);
                    }
                }
            }
        }

        let ans = dp[reqs.last().unwrap().cnt];
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
    #[case(3, vec![vec![2,2],vec![0,0]], 2)]
    #[case(3, vec![vec![2,2],vec![1,1],vec![0,0]], 1)]
    #[case(2, vec![vec![0,0],vec![1,0]], 1)]
    fn case(#[case] n: i32, #[case] requirements: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::number_of_permutations(n, requirements);
        assert_eq!(actual, expected);
    }
}
