//! Solution for https://leetcode.com/problems/all-oone-data-structure
//! 432. All O`one Data Structure

use std::collections::HashMap;
use std::iter;

#[derive(Default)]
struct MyDeque<T> {
    vec: Vec<T>,
    first: usize,
    len: usize,
    pos_of: HashMap<T, usize>,
}

impl<T: Default + Copy + Eq + Hash> MyDeque<T> {
    fn new() -> Self {
        Self::default()
    }

    fn push_back(&mut self, x: T) {
        let pos = self.first + self.len;
        assert!(pos == self.vec.len());
        self.vec.push(x);
        self.len += 1;
        self.pos_of.insert(x, pos);
    }

    fn push_front(&mut self, x: T) {
        if self.first == 0 {
            let to_extend = self.len.max(2);
            self.vec
                .splice(0..0, iter::repeat_with(Default::default).take(to_extend));
            self.first += to_extend;
            for (_, pos) in &mut self.pos_of {
                *pos += to_extend;
            }
        }

        self.first -= 1;
        self.vec[self.first] = x;
        self.len += 1;
        self.pos_of.insert(x, self.first);
    }

    fn pop_back(&mut self) {
        assert!(self.len > 0);
        let x = self.vec.pop().unwrap();
        self.pos_of.remove(&x);
        self.len -= 1;
    }

    fn pop_front(&mut self) {
        assert!(self.len > 0);
        let x = self.vec[self.first];
        self.pos_of.remove(&x);
        self.first += 1;
        self.len -= 1;
        // if self.len * 3 < self.vec.len() {
        //     let to_remove = self.vec.len() / 3;
        //     self.vec.drain(0..to_remove);
        //     self.first -= to_remove;
        // }
    }

    fn get(&self, pos: usize) -> T {
        assert!(pos < self.len);
        self.vec[self.first + pos]
    }

    fn get_pos_of(&self, x: T) -> usize {
        *self.pos_of.get(&x).unwrap() - self.first
    }

    fn swap(&mut self, mut i: usize, mut j: usize) {
        if i == j {
            return;
        }
        assert!(i < self.len);
        assert!(j < self.len);
        i += self.first;
        j += self.first;
        let x_i = self.vec[i];
        let x_j = self.vec[j];
        self.pos_of.insert(x_i, j);
        self.pos_of.insert(x_j, i);
        self.vec.swap(i, j);
    }

    fn back(&self) -> T {
        assert!(self.len > 0);
        self.vec[self.first + self.len - 1]
    }

    fn front(&self) -> T {
        assert!(self.len > 0);
        self.vec[self.first]
    }
}

#[derive(Default)]
struct AllOne {
    hash_to_str: HashMap<u64, String>,
    cnt_of_key: HashMap<u64, usize>,
    deque: MyDeque<u64>,
    cnt_with_cnt_less_than: Vec<i32>,
    cnt_with_cnt_less_than_offset: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        Self::default()
    }

    fn inc(&mut self, key: String) {
        let h = get_hash(&key);
        self.hash_to_str.insert(h, key);

        let cur_cnt = self.cnt_of_key.entry(h).or_insert(0);
        *cur_cnt += 1;

        while *cur_cnt >= self.cnt_with_cnt_less_than.len() {
            self.cnt_with_cnt_less_than.push(0);
        }

        if *cur_cnt == 1 {
            self.deque.push_front(h);
            self.cnt_with_cnt_less_than_offset += 1;
            self.cnt_with_cnt_less_than[0] -= 1;
            self.cnt_with_cnt_less_than[1] -= 1;
            return;
        }

        let pos = self.deque.get_pos_of(h);
        let pos_last =
            self.cnt_with_cnt_less_than[*cur_cnt] + self.cnt_with_cnt_less_than_offset - 1;
        assert!(pos_last >= 0);
        self.deque.swap(pos, pos_last as usize);
        self.cnt_with_cnt_less_than[*cur_cnt] -= 1;
    }

    fn dec(&mut self, key: String) {
        let h = get_hash(&key);

        let cur_cnt = *self.cnt_of_key.get(&h).unwrap();

        let pos = self.deque.get_pos_of(h);
        let pos_first = self.cnt_with_cnt_less_than[cur_cnt] + self.cnt_with_cnt_less_than_offset;
        assert!(pos_first >= 0);
        self.deque.swap(pos, pos_first as usize);
        if cur_cnt == 1 {
            self.cnt_of_key.remove(&h);
            self.deque.pop_front();
            self.cnt_with_cnt_less_than_offset -= 1;
            self.cnt_with_cnt_less_than[0] += 1;
            self.cnt_with_cnt_less_than[1] += 1;
        } else {
            self.cnt_with_cnt_less_than[cur_cnt] += 1;
            self.cnt_of_key.insert(h, cur_cnt - 1);
        }
    }

    fn get_max_key(&self) -> String {
        if self.deque.len == 0 {
            return String::new();
        }
        let h = self.deque.back();
        self.hash_to_str.get(&h).unwrap().clone()
    }

    fn get_min_key(&self) -> String {
        if self.deque.len == 0 {
            return String::new();
        }
        let h = self.deque.front();
        self.hash_to_str.get(&h).unwrap().clone()
    }
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
