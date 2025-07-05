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
