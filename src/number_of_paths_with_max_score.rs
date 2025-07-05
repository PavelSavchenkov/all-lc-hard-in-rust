//! Solution for https://leetcode.com/problems/number-of-paths-with-max-score
//! 1301. Number of Paths with Max Score

const di: [usize; 3] = [0, 1, 1];
const dj: [usize; 3] = [1, 0, 1];

impl Solution {
    pub fn paths_with_max_score(g: Vec<String>) -> Vec<i32> {
        let g = to_u8_vec(&g);
        let n = g.len();
        let m = g[0].len();

        assert!(g[n - 1][m - 1] == b'S');
        assert!(g[0][0] == b'E');

        let mut dp = vec![vec![(-1, Num::new(0)); m]; n];
        dp[0][0] = (0, Num::new(1));
        for i in 0..n {
            for j in 0..m {
                let cur_dp = dp[i][j];
                if cur_dp.0 == -1 {
                    continue;
                }
                for d in 0..3 {
                    let ni = i + di[d];
                    let nj = j + dj[d];
                    if ni < n && nj < m && g[ni][nj] != b'X' {
                        let mut nsum = cur_dp.0;
                        let ch = g[ni][nj];
                        if ch.is_ascii_digit() {
                            nsum += (g[ni][nj] - b'0') as i32;
                        }
                        if nsum < dp[ni][nj].0 {
                            continue;
                        }
                        if nsum > dp[ni][nj].0 {
                            dp[ni][nj] = (nsum, cur_dp.1);
                        } else {
                            dp[ni][nj].1 += cur_dp.1;
                        }
                    }
                }
            }
        }
        let (sum, cnt) = dp[n - 1][m - 1];
        if sum == -1 {
            return vec![0, 0];
        }
        vec![sum, cnt.val as i32]
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
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

    fn pow(&self, p: u64) -> NumMod<M> {
        NumMod::new(Self::pow_mod(self.val, p))
    }

    fn pow_mod(mut x: u32, mut p: u64) -> u32 {
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
        Self::pow_mod(x, (MOD - 2) as u64)
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
    #[case(todo!("[\"E23\",\"2X2\",\"12S\"]\r"), todo!("[7,1]\r"))]
    #[case(todo!("[\"E12\",\"1X1\",\"21S\"]\r"), todo!("[4,2]\r"))]
    #[case(todo!("[\"E11\",\"XXX\",\"11S\"]\r"), todo!("[0,0]\r"))]
    fn case(#[case] board: Vec<String>, #[case] expected: Vec<i32>) {
        let actual = Solution::paths_with_max_score(board);
        assert_eq!(actual, expected);
    }
}
