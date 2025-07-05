//! Solution for https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed
//! 381. Insert Delete GetRandom O(1) - Duplicates allowed

use rand::{thread_rng, Rng};
use std::collections::HashMap;

struct Item {
    val: i32,
    deleted: bool,
}

#[derive(Default)]
struct RandomizedCollection {
    cnt_alive: HashMap<i32, usize>,
    unprocesses_deletions: HashMap<i32, usize>,
    total_alive: usize,
    vec: Vec<Item>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, val: i32) -> bool {
        self.total_alive += 1;
        self.vec.push(Item {
            val,
            deleted: false,
        });
        let cnt = self.cnt_alive.entry(val).or_insert(0);
        *cnt += 1;
        *cnt == 1
    }

    fn remove(&mut self, val: i32) -> bool {
        let cnt = self.cnt_alive.entry(val).or_insert(0);
        if *cnt == 0 {
            return false;
        }
        *cnt -= 1;
        *self.unprocesses_deletions.entry(val).or_insert(0) += 1;
        self.total_alive -= 1;
        true
    }

    fn rebuild(&mut self) {
        let mut new_self = Self::new();
        for item in &self.vec {
            if item.deleted {
                continue;
            }
            let cnt = self.unprocesses_deletions.entry(item.val).or_insert(0);
            if *cnt > 0 {
                *cnt -= 1;
                continue;
            }
            new_self.insert(item.val);
        }
        *self = new_self;
    }

    fn get_random(&mut self) -> i32 {
        if self.total_alive * 2 < self.vec.len() {
            self.rebuild();
        }

        assert!(!self.vec.is_empty());

        loop {
            let i = thread_rng().gen_range(0..self.vec.len());
            let item = &mut self.vec[i];
            if item.deleted {
                continue;
            }
            let cnt_unprocessed_deletions = self.unprocesses_deletions.entry(item.val).or_insert(0);
            if *cnt_unprocessed_deletions > 0 {
                item.deleted = true;
                *cnt_unprocessed_deletions -= 1;
                continue;
            }
            return item.val;
        }
    }
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
