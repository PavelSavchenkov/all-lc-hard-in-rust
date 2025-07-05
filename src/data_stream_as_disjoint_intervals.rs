//! Solution for https://leetcode.com/problems/data-stream-as-disjoint-intervals
//! 352. Data Stream as Disjoint Intervals

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

#[derive(Default)]
struct SummaryRanges {
    set: BTreeSet<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    fn new() -> Self {
        Self::default()
    }

    fn add_num(&mut self, value: i32) {
        {
            let seg = self.get_seg_with(value);
            if seg.is_some() {
                return;
            }
        }

        let mut l = value;
        let mut r = value;
        {
            let prev = self.get_seg_with(l - 1);
            if prev.is_some() {
                let prev = prev.unwrap();
                l = prev.0;
                self.set.remove(&prev);
            }
        }
        {
            let next = self.get_seg_with(r + 1);
            if next.is_some() {
                let next = next.unwrap();
                r = next.1;
                self.set.remove(&next);
            }
        }
        self.set.insert((l, r));
    }

    fn get_seg_with(&self, val: i32) -> Option<(i32, i32)> {
        let range = (Unbounded, Excluded((val, i32::MAX)));
        let it = self.set.range(range);
        let last = it.last();
        if last.is_none() {
            return None;
        }
        let (l, r) = last.unwrap().clone();
        assert!(l <= val);
        if r >= val {
            Some((l, r))
        } else {
            None
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        for &(l, r) in &self.set {
            ans.push(vec![l, r]);
        }
        ans
    }
}

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * let obj = SummaryRanges::new();
 * obj.add_num(value);
 * let ret_2: Vec<Vec<i32>> = obj.get_intervals();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
