//! Solution for https://leetcode.com/problems/finding-mk-average
//! 1825. Finding MK Average

use std::collections::{BTreeSet, VecDeque};

#[derive(Default)]
struct MKAverage {
    m: usize,
    k: usize,
    small: BTreeSet<(i32, usize)>,
    mid: BTreeSet<(i32, usize)>,
    sum_mid: i64,
    big: BTreeSet<(i32, usize)>,
    ptr: usize,
    q: VecDeque<(i32, usize)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        Self {
            m: m as usize,
            k: k as usize,
            ..Default::default()
        }
    }

    fn add_element(&mut self, num: i32) {
        let elem = (num, self.ptr);
        self.ptr += 1;
        self.small.insert(elem);
        self.q.push_back(elem);
        self.norm();
    }

    fn len(&self) -> usize {
        self.small.len() + self.mid.len() + self.big.len()
    }

    fn remove_mid(&mut self, elem: &(i32, usize)) {
        if self.mid.remove(elem) {
            self.sum_mid -= elem.0 as i64;
        }
    }

    fn insert_mid(&mut self, elem: &(i32, usize)) {
        if self.mid.insert(*elem) {
            self.sum_mid += elem.0 as i64;
        }
    }

    fn norm(&mut self) {
        while self.len() > self.m {
            let elem = self.q.pop_front().unwrap();
            self.small.remove(&elem);
            self.remove_mid(&elem);
            self.big.remove(&elem);
        }
        if self.len() < self.m {
            return;
        }
        while self.small.len() > self.k {
            let elem = self.small.pop_last().unwrap();
            self.insert_mid(&elem);
        }
        while self.big.len() > self.k {
            let elem = self.big.pop_first().unwrap();
            self.insert_mid(&elem);
        }
        while self.mid.len() > self.m - 2 * self.k {
            if self.small.len() < self.k {
                let elem = *self.mid.first().unwrap();
                self.remove_mid(&elem);
                self.small.insert(elem);
            } else {
                assert!(self.big.len() < self.k);
                let elem = *self.mid.last().unwrap();
                self.remove_mid(&elem);
                self.big.insert(elem);
            }
        }

        assert!(self.small.len() == self.k);
        assert!(self.big.len() == self.k);
        assert!(self.mid.len() == self.m - 2 * self.k);

        loop {
            let mut did_swap = false;
            if self.small.last().unwrap() > self.mid.first().unwrap() {
                let a = self.small.pop_last().unwrap();
                let b = *self.mid.first().unwrap();
                self.remove_mid(&b);
                self.small.insert(b);
                self.insert_mid(&a);
                did_swap = true;
            }
            if self.mid.last().unwrap() > self.big.first().unwrap() {
                let a = *self.mid.last().unwrap();
                self.remove_mid(&a);
                let b = self.big.pop_first().unwrap();
                self.insert_mid(&b);
                self.big.insert(a);
                did_swap = true;
            }
            if !did_swap {
                break;
            }
        }
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.len() < self.m {
            return -1;
        }
        (self.sum_mid / (self.m - 2 * self.k) as i64) as i32
    }
}

/**
 * Your MKAverage object will be instantiated and called as such:
 * let obj = MKAverage::new(m, k);
 * obj.add_element(num);
 * let ret_2: i32 = obj.calculate_mk_average();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
