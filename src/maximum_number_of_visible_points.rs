//! Solution for https://leetcode.com/problems/maximum-number-of-visible-points
//! 1610. Maximum Number of Visible Points

const EPS: f64 = 1e-10;

impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let mut ps: Vec<_> = points.iter().map(|p| Point::new(&p)).collect();
        let location = Point::new(&location);
        for p in &mut ps {
            *p -= location;
        }

        let cnt_zeros = ps.iter().filter(|&&p| p == Point::ZERO).count();
        ps = ps.into_iter().filter(|&p| p != Point::ZERO).collect();

        ps.sort_by(|&p, &q| {
            if p.quad() != q.quad() {
                return p.quad().cmp(&q.quad());
            }
            let prod = p % q;
            0.cmp(&prod)
        });

        let mut angles = Vec::new();
        for &p in &ps {
            angles.push(p.angle());
        }
        for &p in &ps {
            angles.push(p.angle() + 2.0 * f64::consts::PI);
        }
        ps.extend_from_within(..);

        // eprintln!("{:#?}", ps);
        // eprintln!("{:#?}", angles);

        let mut ans = 0;
        let angle = angle as f64 / 360.0 * 2.0 * f64::consts::PI;
        let mut r = 0;
        for i in 0..ps.len() {
            r = r.max(i);
            while r < ps.len() && angles[r] - angles[i] <= angle + EPS {
                r += 1;
            }
            ans = ans.max(r - i);
        }
        ans += cnt_zeros;
        ans as i32
    }
}

use core::f64;
use std::cmp::Ordering;
use std::ops::{Mul, Rem, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const ZERO: Self = Self { x: 0, y: 0 };

    fn new(v: &Vec<i32>) -> Self {
        Self { x: v[0], y: v[1] }
    }

    fn len2(&self) -> i64 {
        *self * *self
    }

    fn quad(&self) -> u8 {
        if self.x >= 0 && self.y >= 0 {
            return 0;
        }
        if self.x < 0 && self.y >= 0 {
            return 1;
        }
        if self.x <= 0 && self.y <= 0 {
            return 2;
        }
        return 3;
    }

    fn angle(&self) -> f64 {
        let mut a = (self.y as f64).atan2(self.x as f64);
        if a < 0.0 {
            a += 2.0 * f64::consts::PI;
        }
        a
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

impl Mul for Point {
    type Output = i64;

    fn mul(self, other: Self) -> Self::Output {
        self.x as i64 * other.x as i64 + self.y as i64 * other.y as i64
    }
}

impl Rem for Point {
    type Output = i64;

    fn rem(self, other: Self) -> Self::Output {
        self.x as i64 * other.y as i64 - self.y as i64 * other.x as i64
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![2,1],vec![2,2],vec![3,3]], 90, vec![1,1], 3)]
    #[case(vec![vec![2,1],vec![2,2],vec![3,4],vec![1,1]], 90, vec![1,1], 4)]
    #[case(vec![vec![1,0],vec![2,1]], 13, vec![1,1], 1)]
    fn case(
        #[case] points: Vec<Vec<i32>>,
        #[case] angle: i32,
        #[case] location: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::visible_points(points, angle, location);
        assert_eq!(actual, expected);
    }
}
