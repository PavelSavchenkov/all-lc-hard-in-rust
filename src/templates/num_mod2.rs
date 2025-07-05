//! Solution for https://leetcode.com/problems/longest-common-subpath
//! 1923. Longest Common Subpath

use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

type Num = NumMod<1_000_000_007, 1_000_000_019>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NumMod<const M1: u32, const M2: u32> {
    vals: [u32; 2],
}

impl<const M1: u32, const M2: u32> NumMod<M1, M2> {
    // doesn't work
    const static_assert: () = {
        assert!(M1 < (1 << 31));
        assert!(M2 < (1 << 31));
    };

    fn new(x: u32) -> Self {
        NumMod {
            vals: [x % M1, x % M2],
        }
    }

    fn new2(x: u32, y: u32) -> Self {
        NumMod {
            vals: [x % M1, y % M2],
        }
    }

    // x and y are already normalized
    fn safe_add_mod<const M: u32>(mut x: u32, y: u32) -> u32 {
        x += y;
        if x >= M {
            x -= M;
        }
        x
    }

    fn safe_sub_mod<const M: u32>(mut x: u32, y: u32) -> u32 {
        x += M - y;
        if x >= M {
            x -= M;
        }
        x
    }

    fn safe_mul_mod<const M: u32>(x: u32, y: u32) -> u32 {
        ((x as u64 * y as u64) % M as u64) as u32
    }
}

impl<const M1: u32, const M2: u32> Add for NumMod<M1, M2> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            vals: [
                Self::safe_add_mod::<M1>(self.vals[0], other.vals[0]),
                Self::safe_add_mod::<M2>(self.vals[1], other.vals[1]),
            ],
        }
    }
}

impl<const M1: u32, const M2: u32> AddAssign for NumMod<M1, M2> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<const M1: u32, const M2: u32> Sub for NumMod<M1, M2> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            vals: [
                Self::safe_sub_mod::<M1>(self.vals[0], other.vals[0]),
                Self::safe_sub_mod::<M2>(self.vals[1], other.vals[1]),
            ],
        }
    }
}

impl<const M1: u32, const M2: u32> SubAssign for NumMod<M1, M2> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<const M1: u32, const M2: u32> Mul for NumMod<M1, M2> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            vals: [
                Self::safe_mul_mod::<M1>(self.vals[0], other.vals[0]),
                Self::safe_mul_mod::<M2>(self.vals[1], other.vals[1]),
            ],
        }
    }
}

impl<const M1: u32, const M2: u32> MulAssign for NumMod<M1, M2> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
