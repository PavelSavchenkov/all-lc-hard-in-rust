//! Solution for https://leetcode.com/problems/sum-of-total-strength-of-wizards
//! 2281. Sum of Total Strength of Wizards

fn calc_prev(a: &Vec<i32>, strict: bool) -> Vec<usize> {
    let n = a.len();
    let mut st = Vec::new();
    let mut len = vec![0; n];
    for i in 0..n {
        while !st.is_empty() {
            let head = a[*st.last().unwrap()];
            if strict {
                if head < a[i] {
                    break;
                }
            } else {
                if head <= a[i] {
                    break;
                }
            }
            st.pop();
        }
        len[i] = if st.is_empty() {
            i
        } else {
            i - *st.last().unwrap() - 1
        };
        st.push(i);
    }
    len
}

impl Solution {
    pub fn total_strength(mut a: Vec<i32>) -> i32 {
        let n = a.len();

        let prev = calc_prev(&a, true);

        a.reverse();
        let mut next = calc_prev(&a, false);
        next.reverse();
        a.reverse();

        let mut pref = vec![Num::new(0); n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + Num::new(a[i] as u32);
        }

        let mut pref_pref = vec![Num::new(0); pref.len() + 1];
        for i in 0..pref.len() {
            pref_pref[i + 1] = pref_pref[i] + pref[i];
        }

        let mut ans = Num::new(0);
        for i in 0..n {
            let left_len = prev[i];
            let right_len = next[i];

            // sum over (pref[r] - pref[l]); l = i - left_len ... i; r = i + 1 ... i + right_len + 1
            let sum_left = pref_pref[i + 1] - pref_pref[i - left_len];
            let sum_right = pref_pref[i + right_len + 1 + 1] - pref_pref[i + 1];
            let cnt_left = Num::new((left_len + 1) as u32);
            let cnt_right = Num::new((right_len + 1) as u32);

            let cur = sum_right * cnt_left - sum_left * cnt_right;
            // eprintln!("a[i] = {}, cur = {:#?}", a[i], cur.val);
            ans += cur * Num::new(a[i] as u32);
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
    #[case(vec![1,3,1,2], 44)]
    #[case(vec![5,4,6], 213)]
    fn case(#[case] strength: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::total_strength(strength);
        assert_eq!(actual, expected);
    }
}
