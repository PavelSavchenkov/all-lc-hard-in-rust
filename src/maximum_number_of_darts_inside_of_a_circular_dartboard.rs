//! Solution for https://leetcode.com/problems/maximum-number-of-darts-inside-of-a-circular-dartboard
//! 1453. Maximum Number of Darts Inside of a Circular Dartboard

const EPS: f64 = 1e-10;

impl Solution {
    pub fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
        let ps: Vec<_> = darts
            .iter()
            .map(|p| Point {
                x: p[0] as f64,
                y: p[1] as f64,
            })
            .collect();
        let r = r as f64;
        let n = ps.len();

        let mut ans = 1;
        for i in 0..n {
            for j in 0..i {
                let a = ps[i];
                let b = ps[j];

                let d2 = (a - b).len2();
                let mut w = r * r - d2 / 4.0;
                if w < 0.0 {
                    continue;
                }
                w = w.sqrt();
                let m = (a + b) * 0.5;

                let mut v = (b - a) * 0.5;
                v = v.norm();
                assert!((1.0 - v.len2()).abs() < EPS);
                v = Point { x: -v.y, y: v.x };

                for it in 0..2 {
                    let c = m + v * w;

                    let mut cnt = 0;
                    for k in 0..n {
                        let dist2 = (ps[k] - c).len2();
                        if dist2 <= r * r + EPS {
                            cnt += 1;
                        }
                    }

                    ans = ans.max(cnt);

                    v = Point { x: 0.0, y: 0.0 } - v;
                }
            }
        }
        ans
    }
}

use std::ops::{Add, AddAssign, Mul, Rem, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn len2(&self) -> f64 {
        *self * *self
    }

    fn norm(&self) -> Self {
        let len = self.len2().sqrt();
        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Mul for Point {
    type Output = f64;

    fn mul(self, other: Self) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, coef: f64) -> Self::Output {
        Self {
            x: self.x * coef,
            y: self.y * coef,
        }
    }
}

impl Rem for Point {
    type Output = f64;

    fn rem(self, other: Self) -> Self::Output {
        self.x * other.y - self.y * other.x
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![-2,0],vec![2,0],vec![0,2],vec![0,-2]], 2, 4)]
    #[case(vec![vec![-3,0],vec![3,0],vec![2,6],vec![5,4],vec![0,9],vec![7,8]], 5, 5)]
    fn case(#[case] darts: Vec<Vec<i32>>, #[case] r: i32, #[case] expected: i32) {
        let actual = Solution::num_points(darts, r);
        assert_eq!(actual, expected);
    }
}
