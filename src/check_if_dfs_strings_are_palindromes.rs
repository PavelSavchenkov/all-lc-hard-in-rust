//! Solution for https://leetcode.com/problems/check-if-dfs-strings-are-palindromes
//! 3327. Check if DFS Strings Are Palindromes

fn dfs(
    v: usize,
    g: &Vec<Vec<usize>>,
    s: &Vec<u8>,
    p: &Vec<Num>,
    hash: &mut Vec<Num>,
    hash_rev: &mut Vec<Num>,
) -> usize {
    let mut sz = 0;
    for &to in &g[v] {
        let sz_to = dfs(to, g, s, p, hash, hash_rev);
        let hash_to = hash[to];
        let hash_rev_to = hash_rev[to];
        hash[v] += hash_to * p[sz];
        hash_rev[v] = hash_rev[v] * p[sz_to] + hash_rev_to;
        sz += sz_to;
    }
    hash[v] += p[sz] * Num::new((s[v] - b'a') as u32);
    hash_rev[v] = hash_rev[v] * p[1] + Num::new((s[v] - b'a') as u32);
    sz += 1;
    sz
}

const P: u32 = 239;

impl Solution {
    pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
        let n = parent.len();
        let s = to_u8(&s);

        let mut g = vec![Vec::new(); n];
        for v in 1..n {
            g[parent[v] as usize].push(v);
        }

        let mut hash = vec![Num::new(0); n];
        let mut hash_rev = vec![Num::new(0); n];
        let mut p = vec![Num::new(1); n + 1];
        for i in 1..p.len() {
            p[i] = p[i - 1] * Num::new(P);
        }

        dfs(0, &g, &s, &p, &mut hash, &mut hash_rev);

        let mut ans = Vec::with_capacity(n);
        for v in 0..n {
            ans.push(hash[v] == hash_rev[v]);
        }
        ans
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
    #[case(vec![-1,0,0,1,1,2], "aababa", vec![true,true,false,true,true,true])]
    #[case(vec![-1,0,0,0,0], "aabcb", vec![true,true,true,true,true])]
    fn case(#[case] parent: Vec<i32>, #[case] s: String, #[case] expected: Vec<bool>) {
        let actual = Solution::find_answer(parent, s);
        assert_eq!(actual, expected);
    }
}
