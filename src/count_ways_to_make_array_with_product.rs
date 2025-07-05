//! Solution for https://leetcode.com/problems/count-ways-to-make-array-with-product
//! 1735. Count Ways to Make Array With Product

fn power_in(mut x: usize, p: usize) -> usize {
    let mut ans = 0;
    while x % p == 0 {
        ans += 1;
        x /= p;
    }
    ans
}

#[derive(Copy, Clone)]
struct Query {
    n: usize,
    k: usize,
}

impl Solution {
    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        let qs: Vec<_> = queries
            .into_iter()
            .map(|q| Query {
                n: q[0] as usize,
                k: q[1] as usize,
            })
            .collect();

        let max_k = qs.iter().fold(0, |acc, &e| acc.max(e.k));
        let max_n = qs.iter().fold(0, |acc, &e| acc.max(e.n));

        let mut max_pw = 0;
        let mut is_prime = vec![true; max_k + 1];
        let mut powers = vec![Vec::new(); max_k + 1];
        for p in 2..=max_k {
            if is_prime[p] {
                for x in (p..=max_k).step_by(p) {
                    let pw = power_in(x, p);
                    max_pw = max_pw.max(pw);
                    powers[x].push(pw);
                }
                for x in (p + p..=max_k).step_by(p) {
                    is_prime[x] = false;
                }
            }
        }

        // actually just binom
        let mut ways = vec![vec![Num::new(0); max_pw + 1]; max_n + 1];
        ways[0][0] = Num::new(1);
        for n in 0..max_n {
            for have in 0..=max_pw {
                let cur_ways = ways[n][have];
                for now in 0..=max_pw - have {
                    ways[n + 1][have + now] += cur_ways;
                }
            }
        }

        let mut ans = Vec::new();
        for &q in &qs {
            let mut res = Num::new(1);
            for &p in &powers[q.k] {
                res *= ways[q.n][p];
            }
            ans.push(res.val as i32);
        }
        ans
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
    #[case(vec![vec![2,6],vec![5,1],vec![73,660]], vec![4,1,50734910])]
    #[case(vec![vec![1,1],vec![2,2],vec![3,3],vec![4,4],vec![5,5]], vec![1,2,3,10,5])]
    fn case(#[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::ways_to_fill_array(queries);
        assert_eq!(actual, expected);
    }
}
