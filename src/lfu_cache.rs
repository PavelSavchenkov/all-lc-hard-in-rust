//! Solution for https://leetcode.com/problems/lfu-cache
//! 460. LFU Cache

use std::collections::{HashMap, VecDeque};

#[derive(Default)]
struct Helper {
    freq_of_key: HashMap<i32, usize>,   // key --> freq
    keys_with_freq: Vec<VecDeque<i32>>, // freq --> key0, key1, ..., last_used_key (maybe invalidated)
    min_freq: usize,
    capacity: usize,
    size: usize,
}

impl Helper {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            ..Self::default()
        }
    }

    // returns which key should be removed
    fn use_key(&mut self, key: i32) -> Option<i32> {
        let freq = self.freq_of_key.entry(key).or_insert(0);
        *freq += 1;
        let freq = *freq;
        let mut key_to_remove = None;
        if freq == 1 {
            if self.size == self.capacity {
                loop {
                    while let Some(key_cand) = self.keys_with_freq[self.min_freq].pop_front() {
                        let real_freq = self.freq_of_key.get(&key_cand);
                        if real_freq.is_none() {
                            continue;
                        }
                        let real_freq = *real_freq.unwrap();
                        if real_freq == self.min_freq {
                            self.size -= 1;
                            assert!(key != key_cand);
                            self.freq_of_key.remove(&key_cand);
                            key_to_remove = Some(key_cand);
                            break;
                        }
                    }
                    if key_to_remove.is_some() {
                        break;
                    }
                    self.min_freq += 1;
                }
            }
            self.size += 1;
        }
        assert!(self.size <= self.capacity);
        while freq >= self.keys_with_freq.len() {
            self.keys_with_freq.push(VecDeque::new());
        }
        self.keys_with_freq[freq].push_back(key);
        self.min_freq = self.min_freq.min(freq);
        assert!(self.size <= self.capacity);
        key_to_remove
    }
}

struct LFUCache {
    helper: Helper,
    map: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            helper: Helper::new(capacity as usize),
            map: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&val) = self.map.get(&key) {
            let to_remove = self.helper.use_key(key);
            assert!(to_remove.is_none());
            return val;
        } else {
            return -1;
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let to_remove = self.helper.use_key(key);
        if let Some(key_remove) = to_remove {
            assert!(key_remove != key);
            self.map.remove(&key_remove);
        }
        self.map.insert(key, value);
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
