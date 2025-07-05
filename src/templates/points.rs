use core::f64;
use std::cmp::Ordering;
use std::ops::{Mul, Rem, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const ZERO: Self = Self{ x: 0, y: 0 };

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