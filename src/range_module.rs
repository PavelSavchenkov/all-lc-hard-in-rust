//! Solution for https://leetcode.com/problems/range-module
//! 715. Range Module

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included, Unbounded};

#[derive(Default)]
struct RangeModule {
    set: BTreeSet<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {
    fn new() -> Self {
        Self::default()
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let candidates = self.extract_all_touching(left, right);
        let mut L = left;
        let mut R = right;
        for &(l, r) in &candidates {
            L = L.min(l);
            R = R.max(r);
        }
        self.set.insert((L, R));
    }

    fn extract_all_touching(&mut self, left: i32, right: i32) -> Vec<(i32, i32)> {
        let mut res = Vec::new();
        {
            let range = (Included((left, left)), Excluded((right, i32::MAX)));
            let mut it = self.set.range(range);
            loop {
                let seg = it.next();
                if seg.is_none() {
                    break;
                }
                res.push(*seg.unwrap());
            }
        }
        {
            let range = (Unbounded, Excluded((left, left)));
            let mut it = self.set.range(range);
            let seg = it.next_back();
            if let Some(prev) = seg {
                if prev.1 >= left {
                    res.push(*prev);
                }
            }
        }

        for seg in &res {
            self.set.remove(&seg);
        }
        res
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        let range = (Unbounded, Excluded((left, i32::MAX)));
        let mut it = self.set.range(range);
        if let Some(&(l, r)) = it.next_back() {
            return l <= left && right <= r;
        }
        false
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let segs = self.extract_all_touching(left, right);
        for &(l, r) in &segs {
            if left <= l && r <= right {
                continue;
            }
            let l0 = l;
            let r0 = left;
            if l0 < r0 {
                self.set.insert((l0, r0));
            }
            let l1 = right;
            let r1 = r;
            if l1 < r1 {
                self.set.insert((l1, r1));
            }
        }
    }
}

/**
 * Your RangeModule object will be instantiated and called as such:
 * let obj = RangeModule::new();
 * obj.add_range(left, right);
 * let ret_2: bool = obj.query_range(left, right);
 * obj.remove_range(left, right);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
