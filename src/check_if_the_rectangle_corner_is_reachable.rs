//! Solution for https://leetcode.com/problems/check-if-the-rectangle-corner-is-reachable
//! 3235. Check if the Rectangle Corner Is Reachable
//!

const EPS: f64 = 1e-10;

#[derive(Clone, Copy)]
struct Circle {
    c: Point,
    r: i32,
}

impl Circle {
    fn new(v: &Vec<i32>) -> Self {
        Self {
            c: Point::new(v),
            r: v[2],
        }
    }

    fn is_not_outside(&self, p: &Point) -> bool {
        (self.c - *p).len2() <= self.r as i64 * self.r as i64
    }

    fn is_not_inside(&self, p: &Point) -> bool {
        (self.c - *p).len2() >= self.r as i64 * self.r as i64
    }

    fn inter_circle(&self, other: &Self) -> Vec<(f64, f64)> {
        let shift_x = self.c.x as f64;
        let shift_y = self.c.y as f64;

        let x = other.c.x as f64 - shift_x;
        let y = other.c.y as f64 - shift_y;
        let r1 = self.r as f64;
        let r2 = other.r as f64;

        if x * x + y * y > (r1 + r2) * (r1 + r2) {
            return Vec::new();
        }

        let A = -2.0 * x;
        let B = -2.0 * y;
        let C = x * x + y * y - r2 * r2 + r1 * r1;

        let alpha = -C / (A * x + B * y);
        let Ox = alpha * x;
        let Oy = alpha * y;

        let d2 = Ox * Ox + Oy * Oy;
        let coef = (r1 * r1 - d2).sqrt();
        let mut dirX = -y;
        let mut dirY = x;
        let len_dir = (dirX * dirX + dirY * dirY).sqrt();
        dirX /= len_dir;
        dirY /= len_dir;

        let mut ans = Vec::new();
        for go in [-1.0, 1.0] {
            let mut xx = Ox + dirX * go * coef;
            let mut yy = Oy + dirY * go * coef;
            xx += shift_x;
            yy += shift_y;
            ans.push((xx, yy));
        }
        ans
    }

    fn is_inter_seg(&self, seg: &Seg) -> bool {
        if !seg.is_hor {
            return self.swap_xy().is_inter_seg(&Seg {
                is_hor: true,
                ..*seg
            });
        }
        let a = Point {
            x: seg.l,
            y: seg.pos,
        };
        let b = Point {
            x: seg.r,
            y: seg.pos,
        };
        if self.is_not_inside(&a) && self.is_not_outside(&b) {
            return true;
        }
        if self.is_not_outside(&a) && self.is_not_inside(&b) {
            return true;
        }
        if (self.c.y - seg.pos).abs() > self.r {
            return false;
        }
        if seg.l <= self.c.x && self.c.x <= seg.r {
            return true;
        }
        false
    }

    fn swap_xy(&self) -> Self {
        Circle {
            c: Point {
                x: self.c.y,
                y: self.c.x,
            },
            r: self.r,
        }
    }
}

struct Seg {
    l: i32,
    r: i32,
    pos: i32,
    is_hor: bool,
}

impl Solution {
    pub fn can_reach_corner(x_corner: i32, y_corner: i32, circles: Vec<Vec<i32>>) -> bool {
        let cs: Vec<_> = circles.iter().map(|c| Circle::new(&c)).collect();

        let segs = vec![
            Seg {
                l: 0,
                r: x_corner,
                pos: y_corner,
                is_hor: true,
            },
            Seg {
                l: 0,
                r: y_corner,
                pos: 0,
                is_hor: false,
            },
            Seg {
                l: 0,
                r: x_corner,
                pos: 0,
                is_hor: true,
            },
            Seg {
                l: 0,
                r: y_corner,
                pos: x_corner,
                is_hor: false,
            },
        ];

        let mut is_center_outside_box = vec![true; cs.len()];
        for i in 0..cs.len() {
            if cs[i].c.x < 0 || cs[i].c.x > x_corner {
                continue;
            }
            if cs[i].c.y < 0 || cs[i].c.y > y_corner {
                continue;
            }
            is_center_outside_box[i] = false;
        }

        let mut dsu = DSU::new(cs.len() + 4);
        dsu.merge(0, 1);
        dsu.merge(2, 3);
        for i in 0..cs.len() {
            for j in 0..4 {
                if cs[i].is_inter_seg(&segs[j]) {
                    dsu.merge(4 + i, j);
                }
            }
            for j in 0..i {
                let inters = cs[i].inter_circle(&cs[j]);
                for &(x, y) in &inters {
                    if EPS < x
                        && x + EPS <= x_corner as f64
                        && EPS <= y
                        && y + EPS <= y_corner as f64
                    {
                        dsu.merge(4 + i, 4 + j);
                    }
                }
            }
        }

        dsu.get(0) != dsu.get(2)
    }
}

struct DSU {
    p: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self { p: vec![-1; n] }
    }

    fn get(&mut self, x: usize) -> usize {
        if self.p[x] < 0 {
            x
        } else {
            self.p[x] = self.get(self.p[x] as usize) as i32;
            self.p[x] as usize
        }
    }

    fn merge(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.get(x);
        y = self.get(y);
        if x == y {
            return false;
        }
        if -self.p[x] < -self.p[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.p[x] += self.p[y];
        self.p[y] = x as i32;
        true
    }

    fn comp_size(&mut self, x: usize) -> usize {
        let root = self.get(x);
        (-self.p[root]) as usize
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
    #[case(3, 4, vec![vec![2,1,1]], true)]
    #[case(3, 3, vec![vec![1,1,2]], false)]
    #[case(3, 3, vec![vec![2,1,1],vec![1,2,1]], false)]
    #[case(4, 4, vec![vec![5,5,1]], true)]
    #[case(15, 15, vec![vec![1,99,85],vec![99,1,85]], true)]
    fn case(
        #[case] x_corner: i32,
        #[case] y_corner: i32,
        #[case] circles: Vec<Vec<i32>>,
        #[case] expected: bool,
    ) {
        let actual = Solution::can_reach_corner(x_corner, y_corner, circles);
        assert_eq!(actual, expected);
    }
}
