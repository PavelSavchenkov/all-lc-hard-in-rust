//! Solution for https://leetcode.com/problems/minimum-skips-to-arrive-at-meeting-on-time
//! 1883. Minimum Skips to Arrive at Meeting On Time

fn remin(a: &mut Fraction, b: Fraction) {
    if *a > b {
        *a = b;
    }
}

impl Solution {
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        let n = dist.len();

        let mut dp = vec![vec![Fraction::new(i32::MAX as i64, 1); n + 1]; n + 1];
        dp[0][0] = Fraction::new(0, 1);
        for i in 0..n {
            for skips in 0..=i {
                let cur_dp = dp[i][skips];
                let w = Fraction::new(dist[i] as i64, speed as i64);
                // skip
                remin(&mut dp[i + 1][skips + 1], cur_dp + w);
                // do not skip
                let pref = cur_dp.round_up();
                remin(&mut dp[i + 1][skips], pref + w);
            }
        }
        let limit = Fraction::new(hours_before as i64, 1);
        for skips in 0..n {
            if dp[n][skips] <= limit {
                return skips as i32;
            }
        }
        -1
    }
}

use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a != 0 && b != 0 {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
    a + b
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct Fraction {
    p: i64,
    q: i64,
}

impl Fraction {
    fn new(p: i64, q: i64) -> Self {
        Self { p, q }.norm()
    }

    fn norm(&self) -> Self {
        let mut p = self.p;
        let mut q = self.q;
        if q < 0 {
            q = -q;
            p = -p;
        }
        let g = gcd(p.abs(), q.abs());
        p /= g;
        q /= g;
        Self { p, q }
    }

    fn as_float(&self) -> f64 {
        self.p as f64 / self.q as f64
    }

    fn round_up(&self) -> Self {
        let num = (self.p + self.q - 1) / self.q;
        Self::new(num, 1)
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.p * other.q).cmp(&(other.p * self.q))
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let g = gcd(self.q, other.q);
        let Q = (self.q / g) * other.q;
        Self::new(self.p * (other.q / g) + other.p * (self.q / g), Q)
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + Self::new(-other.p, other.q)
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.p * other.p, self.q * other.q)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,2], 4, 2, 1)]
    #[case(vec![7,3,5,5], 2, 10, 2)]
    #[case(vec![7,3,5,5], 1, 10, -1)]
    #[case(vec![2,4,4,9,10], 3, 11, 1)]
    fn case(
        #[case] dist: Vec<i32>,
        #[case] speed: i32,
        #[case] hours_before: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::min_skips(dist, speed, hours_before);
        assert_eq!(actual, expected);
    }
}
