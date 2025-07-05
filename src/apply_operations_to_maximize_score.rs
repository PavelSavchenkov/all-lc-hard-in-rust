//! Solution for https://leetcode.com/problems/apply-operations-to-maximize-score
//! 2818. Apply Operations to Maximize Score

fn calc_left(a: &Vec<usize>, strict: bool) -> Vec<usize> {
    let n = a.len();
    let mut st = Vec::new();
    let mut len = vec![0; n];
    for i in 0..n {
        while !st.is_empty() {
            let last = *st.last().unwrap();
            if strict {
                if a[last] >= a[i] {
                    break;
                }
            } else {
                if a[last] > a[i] {
                    break;
                }
            }
            st.pop();
        }
        len[i] = i + 1;
        if !st.is_empty() {
            len[i] = i - *st.last().unwrap();
        }
        st.push(i);
    }
    len
}

impl Solution {
    pub fn maximum_score(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let M = *a.iter().max().unwrap();
        let mut k = k as u64;

        let mut is_prime = vec![true; M + 1];
        let mut prime_score = vec![0; M + 1];
        for p in 2..=M {
            if is_prime[p] {
                for x in (p + p..=M).step_by(p) {
                    is_prime[x] = false;
                    prime_score[x] += 1;
                }
                prime_score[p] = 1;
            }
        }

        let mut scores = Vec::new();
        for i in 0..n {
            scores.push(prime_score[a[i]]);
        }
        let len_left = calc_left(&scores, true);
        scores.reverse();
        let mut len_right = calc_left(&scores, false);
        len_right.reverse();

        let mut vals = Vec::new();
        for i in 0..n {
            let cnt = len_left[i] as u64 * len_right[i] as u64;
            vals.push((a[i], cnt));
        }
        vals.sort();
        vals.reverse();

        let mut ans = Num::new(1);
        for &(x, cnt) in &vals {
            let x = Num::new(x as u32);
            let pref = cnt.min(k);
            ans *= x.pow(pref);
            k -= pref;
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
    #[case(vec![8,3,9,3,8], 2, 81)]
    #[case(vec![19,12,14,6,10,18], 3, 4788)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::maximum_score(nums, k);
        assert_eq!(actual, expected);
    }
}
