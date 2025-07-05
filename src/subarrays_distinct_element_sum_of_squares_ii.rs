//! Solution for https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-ii
//! 2916. Subarrays Distinct Element Sum of Squares II

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Node {
    sum: u64,
    to_add: u64,
    len: usize,
}

impl Node {
    fn new(val: u64, len: usize) -> Self {
        Self {
            sum: val,
            to_add: 0,
            len,
        }
    }

    fn merge(v0: &Node, v1: &Node) -> Node {
        Self {
            sum: v0.sum + v1.sum,
            to_add: 0,
            len: v0.len + v1.len,
        }
    }

    fn apply(&self, to_add: u64) -> Self {
        Self {
            sum: self.sum + to_add * self.len as u64,
            to_add: self.to_add + to_add,
            len: self.len,
        }
    }

    fn push_to(&self, v: &Self) -> Self {
        v.apply(self.to_add)
    }

    fn remove_push_data(&mut self) {
        self.to_add = 0;
    }
}

struct SegmTreeSum {
    t: Vec<Node>,
    sz: usize,
}

impl SegmTreeSum {
    fn new(n: usize) -> Self {
        let mut this = Self {
            t: Vec::new(),
            sz: 1,
        };
        while this.sz < n {
            this.sz *= 2;
        }
        this.t = vec![Node::new(0, 0); this.sz * 2];
        for v in this.sz..this.sz + n {
            this.t[v] = Node::new(0, 1);
        }
        for v in (1..this.sz).rev() {
            this.upd(v);
        }
        this
    }

    fn upd(&mut self, v: usize) {
        assert!(self.t[v].to_add == 0);
        self.t[v] = Node::merge(&self.t[v * 2], &self.t[v * 2 + 1])
    }

    fn push(&mut self, v: usize) {
        self.t[v * 2] = Node::push_to(&self.t[v], &self.t[v * 2]);
        self.t[v * 2 + 1] = Node::push_to(&self.t[v], &self.t[v * 2 + 1]);
        self.t[v].remove_push_data();
    }

    // [l; r)
    fn run_add_seg(&mut self, l: usize, r: usize, to_add: u64) {
        self.add_seg(1, 0, self.sz, l, r, to_add)
    }

    fn add_seg(&mut self, v: usize, tl: usize, tr: usize, mut l: usize, mut r: usize, to_add: u64) {
        l = l.max(tl);
        r = r.min(tr);
        if l >= r {
            return;
        }

        if l == tl && r == tr {
            self.t[v] = self.t[v].apply(to_add);
            return;
        }

        self.push(v);

        let tm = (tl + tr) / 2;
        self.add_seg(v * 2, tl, tm, l, r, to_add);
        self.add_seg(v * 2 + 1, tm, tr, l, r, to_add);

        self.upd(v);
    }

    fn run_get_sum(&mut self, l: usize, r: usize) -> u64 {
        self.get_sum(1, 0, self.sz, l, r)
    }

    fn get_sum(&mut self, v: usize, tl: usize, tr: usize, mut l: usize, mut r: usize) -> u64 {
        l = l.max(tl);
        r = r.min(tr);
        if l >= r {
            return 0;
        }

        if l == tl && r == tr {
            return self.t[v].sum;
        }

        self.push(v);

        let tm = (tl + tr) / 2;
        let mut ans = 0;
        ans += self.get_sum(v * 2, tl, tm, l, r);
        ans += self.get_sum(v * 2 + 1, tm, tr, l, r);

        self.upd(v);

        ans
    }
}

impl Solution {
    pub fn sum_counts(a: Vec<i32>) -> i32 {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let M = *a.iter().max().unwrap();

        let mut prev = vec![0; M + 1];
        let mut ans = Num::new(0);
        let mut sum_sq: u64 = 0;
        let mut tree = SegmTreeSum::new(n);
        for i in 0..n {
            let j = prev[a[i]];
            assert!(j <= i);

            let old_sum = tree.run_get_sum(j, i);
            sum_sq += 2 * old_sum + (i - j) as u64;
            sum_sq += 1; // [i; i]
            tree.run_add_seg(j, i + 1, 1);

            ans += Num::new((sum_sq % MOD as u64) as u32);

            prev[a[i]] = i + 1;

            // eprintln!("cnt distinct for i = {}", i);
            // for k in 0..n {
            //     let val = tree.run_get_sum(k, k + 1);
            //     eprint!("{} ", val);
            // }
            // eprintln!();
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
    #[case(vec![1,2,1], 15)]
    #[case(vec![2,2], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::sum_counts(nums);
        assert_eq!(actual, expected);
    }
}
