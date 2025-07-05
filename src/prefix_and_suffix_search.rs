//! Solution for https://leetcode.com/problems/prefix-and-suffix-search
//! 745. Prefix and Suffix Search

use std::collections::HashMap;

struct WordFilter {
    id: HashMap<(u64, u64), usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut this = Self { id: HashMap::new() };
        for i in 0..words.len() {
            let word = to_u8(&words[i]);
            let mut prefs = Vec::new();
            for pref in 1..=word.len() {
                let h = get_hash(&word[..pref].to_vec());
                prefs.push(h);
            }
            let mut suffs = Vec::new();
            for suff in 1..=word.len() {
                let h = get_hash(&word[word.len() - suff..].to_vec());
                suffs.push(h);
            }
            for &pref in &prefs {
                for &suff in &suffs {
                    let key = (pref, suff);
                    let max_id = this.id.entry(key).or_insert(0);
                    *max_id = (*max_id).max(i);
                }
            }
        }
        this
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let pref = get_hash(&to_u8(&pref));
        let suff = get_hash(&to_u8(&suff));
        let id = self.id.get(&(pref, suff));
        if id.is_none() {
            return -1;
        }
        *id.unwrap() as i32
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(pref, suff);
 */

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
