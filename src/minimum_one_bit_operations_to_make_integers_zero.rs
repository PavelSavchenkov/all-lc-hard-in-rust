//! Solution for https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero
//! 1611. Minimum One Bit Operations to Make Integers Zero

// dp over known n and hb would be better

use std::collections::HashMap;

#[derive(Default)]
struct Mem {
    high_one: HashMap<(u32, u8), u32>,
    all_zeros: HashMap<(u32, u8), u32>,
}

impl Mem {
    fn new() -> Self {
        Self::default()
    }

    fn get_high_one(&self, key: &(u32, u8)) -> Option<u32> {
        self.high_one.get(key).copied()
    }

    fn get_all_zeroes(&self, key: &(u32, u8)) -> Option<u32> {
        self.all_zeros.get(key).copied()
    }

    fn insert_high_one(&mut self, key: (u32, u8), val: u32) -> u32 {
        self.high_one.insert(key, val);
        val
    }

    fn insert_all_zeroes(&mut self, key: (u32, u8), val: u32) -> u32 {
        self.all_zeros.insert(key, val);
        val
    }
}

fn bit(n: u32, b: u8) -> bool {
    ((n >> b) & 1) == 1
}

fn high_one(n: u32, hb: u8, mem: &mut Mem) -> u32 {
    let key = (n, hb);
    if let Some(val) = mem.get_high_one(&key) {
        return val;
    }
    assert!(n < (1 << (hb + 1)));
    let mut ans;
    if bit(n, hb) {
        if hb == 0 {
            assert!(n == 1);
            ans = 0;
        } else {
            ans = all_zeros(n - (1 << hb), hb - 1, mem);
        }
    } else if hb == 0 {
        if n == 1 {
            ans = 0;
        } else {
            ans = 1;
        }
    } else {
        ans = high_one(n, hb - 1, mem);
        ans += 1;
        ans += all_zeros(1 << (hb - 1), hb - 1, mem);
    }
    mem.insert_high_one(key, ans)
}

fn all_zeros(n: u32, hb: u8, mem: &mut Mem) -> u32 {
    assert!(n < (1 << (hb + 1)));
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    assert!(hb > 0);
    let key = (n, hb);
    if let Some(val) = mem.get_all_zeroes(&key) {
        return val;
    }
    let mut ans;
    if !bit(n, hb) {
        ans = all_zeros(n, hb - 1, mem);
    } else {
        ans = high_one(n - (1 << hb), hb - 1, mem);
        ans += 1;
        ans += all_zeros(1 << (hb - 1), hb - 1, mem);
    }
    mem.insert_all_zeroes(key, ans)
}

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut mem = Mem::new();
        all_zeros(n as u32, 30, &mut mem) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, 2)]
    #[case(6, 4)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::minimum_one_bit_operations(n);
        assert_eq!(actual, expected);
    }
}
