//! Solution for https://leetcode.com/problems/rectangle-area-ii
//! 850. Rectangle Area II

struct Rect {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Rect {
    fn new(v: &Vec<i32>) -> Self {
        Self {
            x1: v[0] as u32,
            y1: v[1] as u32,
            x2: v[2] as u32,
            y2: v[3] as u32,
        }
    }
}

impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let rs: Vec<_> = rectangles.iter().map(|v| Rect::new(&v)).collect();

        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for r in &rs {
            xs.push(r.x1);
            xs.push(r.x2);
            ys.push(r.y1);
            ys.push(r.y2);
        }
        xs.sort();
        xs.dedup();
        ys.sort();
        ys.dedup();

        let mut ans = Num::new(0);
        for i in 0..xs.len() - 1 {
            for j in 0..ys.len() - 1 {
                for r in &rs {
                    if r.x1 <= xs[i] && xs[i + 1] <= r.x2 && r.y1 <= ys[j] && ys[j + 1] <= r.y2 {
                        let dx = xs[i + 1] - xs[i];
                        let dy = ys[j + 1] - ys[j];
                        let area = Num::new(dx) * Num::new(dy);
                        ans += area;
                        break;
                    }
                }
            }
        }
        ans.val as i32
    }
}

use std::cmp::Eq;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

const MOD: u32 = 1_000_000_007;
type Num = NumMod<MOD>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

    fn safe_div_mod(x: u32, y: u32) -> u32 {
        Self::safe_mul_mod(x, Self::inv_mod(y))
    }

    fn pow_mod(mut x: u32, mut p: u32) -> u32 {
        let mut ans = 1;
        while p > 0 {
            if p % 2 == 1 {
                ans = Self::safe_mul_mod(ans, x);
            }
            x = Self::safe_mul_mod(x, x);
            p /= 2;
        }
        ans
    }

    fn inv_mod(x: u32) -> u32 {
        assert!(x != 0);
        Self::pow_mod(x, MOD - 2)
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

impl<const M: u32> Div for NumMod<M> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            val: Self::safe_div_mod(self.val, other.val),
        }
    }
}

impl<const M: u32> DivAssign for NumMod<M> {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,0,2,2],vec![1,0,2,3],vec![1,0,3,1]], 6)]
    #[case(vec![vec![0,0,1000000000,1000000000]], 49)]
    fn case(#[case] rectangles: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::rectangle_area(rectangles);
        assert_eq!(actual, expected);
    }
}
