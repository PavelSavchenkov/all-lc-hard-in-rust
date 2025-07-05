//! Solution for https://leetcode.com/problems/maximum-frequency-stack
//! 895. Maximum Frequency Stack

use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
struct FreqStack {
    set: BTreeSet<(usize, usize, i32)>, // (cnt, last_occ, val)
    list_of: HashMap<i32, Vec<usize>>,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, val: i32) {
        let list = self.list_of.entry(val).or_insert(Vec::new());
        if !list.is_empty() {
            let cnt = list.len();
            let last_occ = *list.last().unwrap();
            self.set.remove(&(cnt, last_occ, val));
        }
        list.push(self.size);
        self.set.insert((list.len(), self.size, val));
        self.size += 1;
    }

    fn pop(&mut self) -> i32 {
        let (_, _, val) = self.set.pop_last().unwrap();
        let list = self.list_of.get_mut(&val).unwrap();
        list.pop();
        if !list.is_empty() {
            let last_occ = *list.last().unwrap();
            self.set.insert((list.len(), last_occ, val));
        }
        val
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
