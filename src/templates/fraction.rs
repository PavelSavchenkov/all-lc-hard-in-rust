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
