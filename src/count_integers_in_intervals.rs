//! Solution for https://leetcode.com/problems/count-integers-in-intervals
//! 2276. Count Integers in Intervals

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included, Unbounded};

#[derive(Default)]
struct CountIntervals {
    set: BTreeSet<(i32, i32)>,
    sum_len: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CountIntervals {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, left: i32, right: i32) {
        // find all segs which intersect ours
        let mut inters = Vec::new();

        // the ones start not earlier than (left, right)
        let range = (Included((left, left)), Unbounded);
        let mut it = self.set.range(range);
        loop {
            let seg = it.next();
            if seg.is_none() {
                break;
            }
            let (l, r) = seg.unwrap().clone();
            if l > right {
                break;
            }
            inters.push((l, r));
        }

        // at most one starts earlier than (left, right)
        let range = (Unbounded::<(i32, i32)>, Excluded((left, left)));
        let mut it = self.set.range(range);
        let seg = it.next_back();
        if !seg.is_none() {
            let (l, r) = seg.unwrap().clone();
            if r >= left {
                inters.push((l, r));
            }
        }

        let mut left = left;
        let mut right = right;
        for (l, r) in inters {
            self.remove_seg((l, r));
            left = left.min(l);
            right = right.max(r);
        }

        self.insert_seg((left, right));
    }

    fn count(&self) -> i32 {
        self.sum_len
    }

    fn insert_seg(&mut self, seg: (i32, i32)) {
        self.sum_len += seg.1 - seg.0 + 1;
        self.set.insert(seg);
    }

    fn remove_seg(&mut self, seg: (i32, i32)) {
        self.sum_len -= seg.1 - seg.0 + 1;
        self.set.remove(&seg);
    }
}

/**
 * Your CountIntervals object will be instantiated and called as such:
 * let obj = CountIntervals::new();
 * obj.add(left, right);
 * let ret_2: i32 = obj.count();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
