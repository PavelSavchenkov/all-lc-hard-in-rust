//! Solution for https://leetcode.com/problems/erect-the-fence
//! 587. Erect the Fence

use std::cmp::Ordering;
use std::ops::{Mul, Rem, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn len2(&self) -> i64 {
        *self * *self
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

impl Solution {
    pub fn outer_trees(ps: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let keep_right_turns = |ps: &Vec<Point>| -> Vec<Point> {
            let mut st = Vec::new();
            for &p in ps {
                while st.len() >= 2 {
                    let n = st.len();
                    let q = st[n - 1];
                    let w = st[n - 2];
                    if (p - w) % (q - w) < 0 {
                        st.pop();
                    } else {
                        break;
                    }
                }
                st.push(p);
            }
            st
        };

        let mut ps: Vec<Point> = ps.iter().map(|v| Point { x: v[0], y: v[1] }).collect();

        ps.sort();
        let mut upper = keep_right_turns(&ps);
        ps.reverse();
        let mut lower = keep_right_turns(&ps);

        upper.pop();
        lower.pop();

        let mut ans = upper;
        ans.extend_from_slice(&lower);

        ans.sort();
        ans.dedup();

        if ps.len() == 1 {
            ans = ps;
        }

        let ans: Vec<_> = ans.iter().map(|&p| vec![p.x, p.y]).collect();

        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,1],vec![2,2],vec![2,0],vec![2,4],vec![3,3],vec![4,2]], vec![vec![1,1],vec![2,0],vec![4,2],vec![3,3],vec![2,4]])]
    #[case(vec![vec![1,2],vec![2,2],vec![4,2]], vec![vec![4,2],vec![2,2],vec![1,2]])]
    fn case(#[case] trees: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::outer_trees(trees);
        assert_eq!(actual, expected);
    }
}
