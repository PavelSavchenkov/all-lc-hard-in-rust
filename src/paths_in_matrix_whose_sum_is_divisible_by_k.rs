//! Solution for https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k
//! 2435. Paths in Matrix Whose Sum Is Divisible by K

impl Solution {
    pub fn number_of_paths(g: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = g.len();
        let m = g[0].len();

        let g: Vec<Vec<usize>> = g
            .iter()
            .map(|row| row.iter().map(|&x| (x as usize) % k).collect())
            .collect();

        let mut dp = vec![vec![vec![Num::new(0); k]; m]; n];
        dp[0][0][g[0][0]] = Num::new(1);
        for i in 0..n {
            for j in 0..m {
                for rem in 0..k {
                    let cur_dp = dp[i][j][rem];
                    if cur_dp.val == 0 {
                        continue;
                    }
                    for (ni, nj) in [(i + 1, j), (i, j + 1)] {
                        if ni < n && nj < m {
                            let mut nrem = rem + g[ni][nj];
                            if nrem >= k {
                                nrem -= k;
                            }
                            dp[ni][nj][nrem] += cur_dp;
                        }
                    }
                }
            }
        }

        dp[n - 1][m - 1][0].val as i32
    }
}

use std::cmp::Eq;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

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
    #[case(vec![vec![5,2,4],vec![3,0,5],vec![0,7,2]], 3, 2)]
    #[case(vec![vec![0,0]], 5, 1)]
    #[case(vec![vec![7,3,4,9],vec![2,3,6,2],vec![2,3,7,0]], 1, 10)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::number_of_paths(grid, k);
        assert_eq!(actual, expected);
    }
}
