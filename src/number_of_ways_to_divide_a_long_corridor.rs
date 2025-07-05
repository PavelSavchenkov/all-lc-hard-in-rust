//! Solution for https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor
//! 2147. Number of Ways to Divide a Long Corridor

impl Solution {
    pub fn number_of_ways(a: String) -> i32 {
        let a: Vec<_> = a
            .as_bytes()
            .iter()
            .map(|&b| if b == b'S' { 1 } else { 0 })
            .collect();
        let n = a.len();

        let mut dp = vec![Num::new(0); n + 1];
        dp[0] = Num::new(1);
        let mut pref = vec![Num::new(0); n + 2];
        let mut seats = VecDeque::<i32>::new();
        seats.push_front(-1);
        for i in 0..=n {
            if seats.len() >= 3 {
                assert!(seats.len() == 3);
                let l = (seats[0] + 1) as usize;
                let r = seats[1] as usize;
                dp[i] = pref[r + 1] - pref[l];
            }

            if i < n && a[i] == 1 {
                seats.push_back(i as i32);
                while seats.len() > 3 {
                    seats.pop_front();
                }
            }
            pref[i + 1] = pref[i] + dp[i];
        }

        dp[n].val as i32
    }
}

use std::cmp::Eq;
use std::collections::VecDeque;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

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
    #[case("SSPPSPS", 3)]
    #[case("PPSPSP", 1)]
    #[case("S", 0)]
    fn case(#[case] corridor: String, #[case] expected: i32) {
        let actual = Solution::number_of_ways(corridor);
        assert_eq!(actual, expected);
    }
}
