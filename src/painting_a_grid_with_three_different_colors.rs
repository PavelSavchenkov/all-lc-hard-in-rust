//! Solution for https://leetcode.com/problems/painting-a-grid-with-three-different-colors
//! 1931. Painting a Grid With Three Different Colors

use std::cmp::Eq;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::vec;

type Num = NumMod<1_000_000_007>;

fn state_to_vec(mut state: usize, state_len: usize) -> Vec<usize> {
    let mut v = Vec::new();
    for it in 0..state_len {
        v.push(state % 3);
        state /= 3;
    }
    v
}

impl Solution {
    pub fn color_the_grid(mm: i32, nn: i32) -> i32 {
        let m = mm as usize;
        let n = nn as usize;

        let cnt_states = usize::pow(3, m as u32);
        let pw3: Vec<usize> = (0..=m).map(|p| usize::pow(3, p as u32)).collect();

        let mut dp = vec![Num::new(0); cnt_states];
        dp[0] = Num::new(1);

        for j in 0..n {
            for i in 0..m {
                let mut ndp = vec![Num::new(0); cnt_states];
                for state in 0..cnt_states {
                    if dp[state] == Num::new(0) {
                        continue;
                    }
                    let state_colors = state_to_vec(state, m);
                    for color in 0..3 {
                        if i > 0 && color == state_colors[i - 1] {
                            continue;
                        }
                        if j > 0 && color == state_colors[i] {
                            continue;
                        }
                        let nstate = state - state_colors[i] * pw3[i] + color * pw3[i];
                        ndp[nstate] += dp[state];
                    }
                }
                dp = ndp;
            }
        }

        dp.iter().fold(Num::new(0), |acc, elem| acc + *elem).val as i32
    }
}

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
    #[case(1, 1, 3)]
    #[case(1, 2, 6)]
    #[case(5, 5, 580986)]
    fn case(#[case] m: i32, #[case] n: i32, #[case] expected: i32) {
        let actual = Solution::color_the_grid(m, n);
        assert_eq!(actual, expected);
    }
}
