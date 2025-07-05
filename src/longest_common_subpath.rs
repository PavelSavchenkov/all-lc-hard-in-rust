//! Solution for https://leetcode.com/problems/longest-common-subpath
//! 1923. Longest Common Subpath

use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

type Num = NumMod<1_000_000_007, 1_000_000_019>;

impl Solution {
    pub fn longest_common_subpath(n: i32, paths: Vec<Vec<i32>>) -> i32 {
        let BASE = Num::new2(u32::pow(10, 8) + 19, u32::pow(10, 8) + 37);

        let max_path_len: usize = paths.iter().map(Vec::len).max().unwrap();
        let m = paths.len();

        let mut p: Vec<Num> = vec![Num::new(1); max_path_len + 1];
        for i in 1..=max_path_len {
            p[i] = BASE * p[i - 1];
        }

        let mut hs: Vec<Vec<Num>> = vec![vec![]; m];
        for i in 0..m {
            let mut h = Num::new(0);
            hs[i] = vec![Num::new(0); paths[i].len() + 1];
            for j in 0..paths[i].len() {
                hs[i][j] = h;
                h += Num::new((paths[i][j] + 1) as u32) * p[j];
            }
            hs[i][paths[i].len()] = h;
        }

        let found = |len: usize| {
            assert!(len > 0);
            let mut was = HashSet::new();
            for i in 0..m {
                let mut new_was = HashSet::new();
                if paths[i].len() < len {
                    return false;
                }
                for j in 0..=paths[i].len() - len {
                    let h = (hs[i][j + len] - hs[i][j]) * p[max_path_len - (j + len - 1)];
                    if i == 0 || was.contains(&h) {
                        new_was.insert(h);
                    }
                }
                was = new_was;
            }
            !was.is_empty()
        };

        let mut L = 0;
        let mut R = max_path_len + 1;
        while L != R - 1 {
            let M = (L + R) / 2;
            if found(M) {
                L = M;
            } else {
                R = M;
            }
        }
        L as i32
    }
}

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

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![vec![0,1,2,3,4],vec![2,3,4],vec![4,0,1,2,3]], 2)]
    #[case(3, vec![vec![0],vec![1],vec![2]], 0)]
    #[case(5, vec![vec![0,1,2,3,4],vec![4,3,2,1,0]], 1)]
    fn case(#[case] n: i32, #[case] paths: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::longest_common_subpath(n, paths);
        assert_eq!(actual, expected);
    }
}
