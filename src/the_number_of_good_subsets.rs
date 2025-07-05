//! Solution for https://leetcode.com/problems/the-number-of-good-subsets
//! 1994. The Number of Good Subsets

use std::array;

fn get_prime_sieve(n: usize) -> Vec<bool> {
    let mut is_p = vec![true; n + 1];
    is_p[0] = false;
    is_p[1] = false;
    for p in 2..=n {
        if !is_p[p] {
            continue;
        }
        for i in (p + p..=n).step_by(p) {
            is_p[i] = false;
        }
    }
    is_p
}

fn factorize(mut n: u32) -> Vec<(u32, u32)> {
    let mut factors = Vec::new();
    let mut i = 2;
    while i * i <= n {
        let p = i;
        let mut k = 0;
        while n % p == 0 {
            n /= p;
            k += 1;
        }
        if k > 0 {
            factors.push((p, k));
        }
        i += 1;
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}

const M: usize = 30;

impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let prime_sieve = get_prime_sieve(M);
        let primes: Vec<_> = (2..=M).filter(|&p| prime_sieve[p]).collect();
        let mut id_prime = vec![-1; M + 1];
        for i in 0..primes.len() {
            id_prime[primes[i]] = i as i32;
        }

        let mut mask_of = vec![0 as i32; M + 1];
        for num in 2..=M {
            let factors = factorize(num as u32);
            let mut mask: i32 = 0;
            for (p, k) in factors {
                if k > 1 {
                    mask = -1;
                    break;
                }
                let i = id_prime[p as usize];
                mask ^= 1 << i;
            }
            mask_of[num] = mask;
        }

        let mut cnt = vec![0; M + 1];
        for num in nums {
            cnt[num as usize] += 1;
        }

        let mut dp = vec![Num::new(0); 1 << primes.len()];
        dp[0] = Num::new(1);
        let full_mask: usize = (1 << primes.len()) - 1;
        for num in 2..=M {
            let mask = mask_of[num];
            if mask == -1 {
                continue;
            }
            let mask = mask as usize;
            let mask_rev = full_mask ^ mask;
            let mut prev_mask = mask_rev as usize;
            let mut ndp = dp.clone();
            loop {
                ndp[mask | prev_mask] += dp[prev_mask] * Num::new(cnt[num]);
                if prev_mask == 0 {
                    break;
                }
                prev_mask = (prev_mask - 1) & mask_rev;
            }
            dp = ndp;
        }

        let mut pw2 = Num::new(1);
        for it in 0..cnt[1] {
            pw2 = pw2 * Num::new(2);
        }
        let mut ans = Num::new(0);
        for mask in 1..=full_mask {
            ans += dp[mask];
        }
        ans *= pw2;

        ans.val as i32
    }
}

use std::cmp::Eq;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::vec;

type Num = NumMod<1_000_000_007>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    #[case(vec![1,2,3,4], 6)]
    #[case(vec![4,2,3,15], 5)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::number_of_good_subsets(nums);
        assert_eq!(actual, expected);
    }
}
