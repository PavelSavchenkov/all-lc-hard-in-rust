//! Solution for https://leetcode.com/problems/best-position-for-a-service-centre
//! 1515. Best Position for a Service Centre

const ITERS: usize = 100;

impl Solution {
    pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        let ps: Vec<_> = positions
            .iter()
            .map(|p| Point {
                x: p[0] as f64,
                y: p[1] as f64,
            })
            .collect();

        let f_xy = |x: f64, y: f64| -> f64 {
            let center = Point { x, y };
            ps.iter().fold(0.0, |acc, e| acc + (*e - center).len())
        };

        let f_x = |x: f64| -> f64 {
            let mut Ly = 0.0;
            let mut Ry = 100.0;
            for it in 0..ITERS {
                let M1 = Ly + (Ry - Ly) / 3.0;
                let M2 = Ly + 2.0 * (Ry - Ly) / 3.0;
                if f_xy(x, M1) < f_xy(x, M2) {
                    Ry = M2;
                } else {
                    Ly = M1;
                }
            }
            f_xy(x, (Ly + Ry) / 2.0)
        };

        let mut Lx = 0.0;
        let mut Rx = 100.0;
        for it in 0..ITERS {
            let M1 = Lx + (Rx - Lx) / 3.0;
            let M2 = Lx + 2.0 * (Rx - Lx) / 3.0;
            if f_x(M1) < f_x(M2) {
                Rx = M2;
            } else {
                Lx = M1;
            }
        }
        f_x((Lx + Rx) / 2.0)
    }
}

use std::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    const ZERO: Self = Self { x: 0.0, y: 0.0 };

    fn len2(&self) -> f64 {
        *self * *self
    }

    fn len(&self) -> f64 {
        self.len2().sqrt()
    }

    fn norm(&self) -> Self {
        let len = self.len();
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

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, coef: f64) -> Self::Output {
        Self {
            x: self.x / coef,
            y: self.y / coef,
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
    #[case(vec![vec![0,1],vec![1,0],vec![1,2],vec![2,1]], 4.00000)]
    #[case(vec![vec![1,1],vec![3,3]], 2.82843)]
    fn case(#[case] positions: Vec<Vec<i32>>, #[case] expected: f64) {
        let actual = Solution::get_min_dist_sum(positions);
        assert!(
            (actual - expected).abs() < 1e-5,
            "Assertion failed: actual {actual:.5} but expected {expected:.5}. Diff is more than 1e-5."
        );
    }
}
