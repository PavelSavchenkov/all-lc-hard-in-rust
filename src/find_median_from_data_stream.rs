//! Solution for https://leetcode.com/problems/find-median-from-data-stream
//! 295. Find Median from Data Stream

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

#[derive(Default)]
struct MedianFinder {
    set: BTreeSet<(i32, usize)>, // (num, query_id); query_id is to avoid duplicates
    median: (i32, usize),        // either true median (odd case), or the first median
    ptr_q: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self::default()
    }

    fn add_num(&mut self, num: i32) {
        let item = (num, self.ptr_q);
        self.ptr_q += 1;

        let mut cnt_bigger = self.set.len() / 2;
        self.set.insert(item);
        if self.set.len() == 1 {
            self.median = item;
            return;
        }
        if item > self.median {
            cnt_bigger += 1;
        }

        let need_cnt_bigger = self.set.len() / 2;
        while cnt_bigger > need_cnt_bigger {
            cnt_bigger -= 1;
            self.median = self.get_next(self.median).unwrap();
        }
        while cnt_bigger < need_cnt_bigger {
            cnt_bigger += 1;
            self.median = self.get_prev(self.median).unwrap();
        }
    }

    fn get_next(&self, item: (i32, usize)) -> Option<(i32, usize)> {
        let range = (Excluded(item), Unbounded::<(i32, usize)>);
        let it = self.set.range(range);
        it.min().cloned()
    }

    fn get_prev(&self, item: (i32, usize)) -> Option<(i32, usize)> {
        let range = (Unbounded::<(i32, usize)>, Excluded(item));
        let it = self.set.range(range);
        it.last().cloned()
    }

    fn find_median(&self) -> f64 {
        if self.set.len() % 2 == 1 {
            return self.median.0 as f64;
        }
        let left = self.median.0 as f64;
        let right = self.get_next(self.median).unwrap().0 as f64;
        (left + right) * 0.5
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
