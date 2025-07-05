//! Solution for https://leetcode.com/problems/longest-duplicate-substring
//! 1044. Longest Duplicate Substring

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let s = to_u8(&s);
        let n = s.len();

        let base = Num::new(239);
        let mut p = vec![Num::new(1); n + 1];
        for i in 1..=n {
            p[i] = p[i - 1] * base;
        }
        let mut pref = vec![Num::new(0); n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + Num::new((s[i] - b'a') as u32) * p[i];
        }

        let substr_hash = |l: usize, r: usize| -> Num {
            assert!(l <= r);
            let mut ans = pref[r] - pref[l];
            ans *= p[n - r];
            ans
        };

        let have_two = |len: usize| -> Option<(usize, usize)> {
            let mut last = HashSet::new();
            for i in 0..=n - len {
                let cur = substr_hash(i, i + len);
                if !last.insert(cur) {
                    return Some((i, i + len));
                }
            }
            None
        };

        let mut L = 0;
        let mut R = n;
        while L + 1 != R {
            let M = (L + R) / 2;
            if have_two(M).is_some() {
                L = M;
            } else {
                R = M;
            }
        }
        if L == 0 {
            return String::new();
        }
        let (l, r) = have_two(L).unwrap();
        from_u8(&s[l..r].to_vec())
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
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

type Num = NumMod<1_000_000_007, 1_000_000_019>;

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
    #[case("banana", "ana")]
    #[case("abcd", "")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::longest_dup_substring(s);
        assert_eq!(actual, expected);
    }
}
