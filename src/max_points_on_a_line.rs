//! Solution for https://leetcode.com/problems/max-points-on-a-line
//! 149. Max Points on a Line

use std::ops::{Rem, Sub};

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
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

impl Rem for Point {
    type Output = i32;

    fn rem(self, other: Self) -> Self::Output {
        self.x * other.y - self.y * other.x
    }
}

fn on_same_line(a: &Point, b: &Point, c: &Point) -> bool {
    let u = *b - *a;
    let v = *b - *c;
    let prod = u % v;
    prod == 0
}

impl Solution {
    pub fn max_points(ps: Vec<Vec<i32>>) -> i32 {
        let ps: Vec<_> = ps.iter().map(|p| Point { x: p[0], y: p[1] }).collect();

        let mut ans = 1;
        for i in 0..ps.len() {
            for j in i + 1..ps.len() {
                let mut cur = 2;
                for k in j + 1..ps.len() {
                    if on_same_line(&ps[i], &ps[j], &ps[k]) {
                        cur += 1;
                    }
                }
                ans = ans.max(cur);
            }
        }
        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,1],vec![2,2],vec![3,3]], 3)]
    #[case(vec![vec![1,1],vec![3,2],vec![5,3],vec![4,1],vec![2,3],vec![1,4]], 4)]
    fn case(#[case] points: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::max_points(points);
        assert_eq!(actual, expected);
    }
}
