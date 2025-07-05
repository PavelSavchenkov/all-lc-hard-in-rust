//! Solution for https://leetcode.com/problems/number-of-increasing-paths-in-a-grid
//! 2328. Number of Increasing Paths in a Grid

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn count_paths(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let mut ij = Vec::new();
        for i in 0..n {
            for j in 0..m {
                ij.push((i, j));
            }
        }
        ij.sort_by_key(|&(i, j)| g[i][j]);

        let mut dp = vec![vec![Num::new(1); m]; n];
        let mut ans = Num::new(0);
        for &(i, j) in &ij {
            let cur_dp = dp[i][j];
            ans += cur_dp;
            for k in 0..4 {
                let ni = i as i32 + di[k];
                let nj = j as i32 + dj[k];
                if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if g[ni][nj] <= g[i][j] {
                    continue;
                }
                dp[ni][nj] += cur_dp;
            }
        }

        ans.val as i32
    }
}

use std::cmp::Eq;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

const MOD: u32 = 1_000_000_007;
type Num = NumMod<MOD>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,1],vec![3,4]], 8)]
    #[case(vec![vec![1],vec![2]], 3)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::count_paths(grid);
        assert_eq!(actual, expected);
    }
}
