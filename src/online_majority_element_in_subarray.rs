//! Solution for https://leetcode.com/problems/online-majority-element-in-subarray
//! 1157. Online Majority Element In Subarray

use rand::Rng;
use std::collections::HashMap;

struct MajorityChecker {
    positions: HashMap<i32, Vec<usize>>,
    a: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MajorityChecker {
    fn new(a: Vec<i32>) -> Self {
        let mut positions = HashMap::new();
        let n = a.len();
        for i in 0..n {
            let x = a[i];
            positions.entry(x).or_insert(Vec::new()).push(i);
        }
        Self { positions, a }
    }

    fn get_cnt(&self, left: usize, right: usize, val: i32) -> usize {
        let pos = self.positions.get(&val).unwrap();
        let l = pos.partition_point(|&i| i < left);
        let r = pos.partition_point(|&i| i <= right);
        r - l
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        let threshold = threshold as usize;
        let mut rng = rand::thread_rng();
        for iter in 0..40 {
            let pos = rng.gen_range(left..=right);
            let val = self.a[pos];
            if self.get_cnt(left, right, val) >= threshold {
                return val;
            }
        }
        -1
    }
}

/**
 * Your MajorityChecker object will be instantiated and called as such:
 * let obj = MajorityChecker::new(arr);
 * let ret_1: i32 = obj.query(left, right, threshold);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
