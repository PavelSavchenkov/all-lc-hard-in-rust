//! Solution for https://leetcode.com/problems/count-ways-to-build-rooms-in-an-ant-colony
//! 1916. Count Ways to Build Rooms in an Ant Colony

struct Binom {
    fact: Vec<Num>,
    inv_fact: Vec<Num>,
}

impl Binom {
    fn new(n: usize) -> Self {
        let mut fact = vec![Num::new(1); n + 1];
        let mut inv_fact = vec![Num::new(1); n + 1];
        for i in 2..=n {
            fact[i] = fact[i - 1] * Num::new(i as u32);
            inv_fact[i] = Num::new(1) / fact[i];
        }
        Self { fact, inv_fact }
    }

    fn get(&self, n: usize, k: usize) -> Num {
        self.fact[n] * self.inv_fact[k] * self.inv_fact[n - k]
    }
}

// (ans, sz)
fn dfs(v: usize, g: &Vec<Vec<usize>>, binom: &Binom) -> (Num, usize) {
    let mut answers = Vec::new();
    let mut sizes = Vec::new();
    for &to in &g[v] {
        let (ans, size) = dfs(to, g, binom);
        answers.push(ans);
        sizes.push(size);
    }

    let mut pref_size = 0;
    let mut ans = Num::new(1);
    for i in 0..answers.len() {
        pref_size += sizes[i];
        let coef = binom.get(pref_size, sizes[i]);
        ans *= coef * answers[i];
    }

    (ans, pref_size + 1)
}

impl Solution {
    pub fn ways_to_build_rooms(par: Vec<i32>) -> i32 {
        let n = par.len();

        let mut g = vec![Vec::new(); n];
        for i in 1..n {
            g[par[i] as usize].push(i);
        }

        let binom = Binom::new(n);
        let ans = dfs(0, &g, &binom).0;
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
    #[case(vec![-1,0,1], 1)]
    #[case(vec![-1,0,0,1,2], 6)]
    fn case(#[case] prev_room: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::ways_to_build_rooms(prev_room);
        assert_eq!(actual, expected);
    }
}
