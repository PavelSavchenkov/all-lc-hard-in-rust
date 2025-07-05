//! Solution for https://leetcode.com/problems/dinner-plate-stacks
//! 1172. Dinner Plate Stacks

use std::collections::BTreeSet;

#[derive(Default)]
struct DinnerPlates {
    stacks: Vec<Vec<i32>>,
    non_empty_stacks: BTreeSet<usize>,
    non_full_stacks: BTreeSet<usize>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        assert!(capacity > 0);
        Self {
            capacity: capacity as usize,
            ..Default::default()
        }
    }

    fn push(&mut self, val: i32) {
        if self.non_full_stacks.is_empty() {
            self.non_full_stacks.insert(self.stacks.len());
            self.stacks.push(Vec::new());
        }
        let i = *self.non_full_stacks.first().unwrap();
        assert!(self.stacks[i].len() < self.capacity);
        self.stacks[i].push(val);
        if self.stacks[i].len() == 1 {
            self.non_empty_stacks.insert(i);
        }
        if self.stacks[i].len() == self.capacity {
            self.non_full_stacks.remove(&i);
        }
    }

    fn pop(&mut self) -> i32 {
        let i = self.non_empty_stacks.last();
        if i.is_none() {
            return -1;
        }
        let i = *i.unwrap();
        self.pop_at_stack(i as i32)
    }

    fn pop_at_stack(&mut self, i: i32) -> i32 {
        let i = i as usize;
        if i >= self.stacks.len() || self.stacks[i].is_empty() {
            return -1;
        }
        if self.stacks[i].len() == self.capacity {
            self.non_full_stacks.insert(i);
        }
        let val = self.stacks[i].pop().unwrap();
        if self.stacks[i].is_empty() {
            self.non_empty_stacks.remove(&i);
        }
        val
    }
}

/**
 * Your DinnerPlates object will be instantiated and called as such:
 * let obj = DinnerPlates::new(capacity);
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.pop_at_stack(index);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
