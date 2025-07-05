//! Solution for https://leetcode.com/problems/longest-substring-of-one-repeating-character
//! 2213. Longest Substring of One Repeating Character

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

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

#[derive(Default)]
struct Segs {
    set: BTreeSet<(usize, usize, u8)>, // [l, r), char
    lens: BTreeSet<(usize, usize)>,    // (len, l), len is to avoid duplicates
}

impl Segs {
    fn new() -> Self {
        Self::default()
    }

    fn add_seg(&mut self, (mut l, mut r, c): (usize, usize, u8)) {
        if r == l {
            return;
        }
        assert!(r > l);

        // try prev
        if l > 0 {
            let seg_left = self.get_seg_with(l - 1);
            if !seg_left.is_none() {
                let (l1, r1, c1) = seg_left.unwrap();
                if c1 == c {
                    assert!(r1 == l);
                    self.del_seg((l1, r1, c1));
                    l = l1;
                }
            }
        }
        // try next
        {
            let seg_right = self.get_seg_with(r);
            if !seg_right.is_none() {
                let (l2, r2, c2) = seg_right.unwrap();
                if c2 == c {
                    assert!(l2 == r);
                    self.del_seg((l2, r2, c2));
                    r = r2;
                }
            }
        }

        self.set.insert((l, r, c));
        self.lens.insert((r - l, l));
    }

    fn del_seg(&mut self, (l, r, c): (usize, usize, u8)) {
        assert!(r > l);
        self.set.remove(&(l, r, c));
        self.lens.remove(&(r - l, l));
    }

    fn get_seg_with(&self, i: usize) -> Option<(usize, usize, u8)> {
        let key = (i, usize::MAX, 0 as u8);
        let range = (Unbounded, Excluded(key));
        let it = self.set.range(range);
        let it = it.last();
        if it.is_none() {
            return None;
        }
        let (l, r, c) = *it.unwrap();
        if l <= i && i < r {
            Some((l, r, c))
        } else {
            None
        }
    }

    fn max_len(&self) -> usize {
        assert!(!self.lens.is_empty());
        let mx = self.lens.last().unwrap();
        mx.0
    }
}

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let s = to_u8(&s);
        let n = s.len();
        let query_chars = to_u8(&query_characters);
        let query_i: Vec<_> = query_indices.iter().map(|&i| i as usize).collect();

        let mut segs = Segs::new();
        {
            let mut i = 0;
            while i < n {
                let mut j = i;
                while j < n && s[j] == s[i] {
                    j += 1;
                }
                segs.add_seg((i, j, s[i]));
                i = j;
            }
        }

        let mut ans = Vec::new();
        for id in 0..query_chars.len() {
            let i = query_i[id];
            let ch = query_chars[id];

            let (l, r, ch_old) = segs.get_seg_with(i).unwrap();
            segs.del_seg((l, r, ch_old));
            segs.add_seg((l, i, ch_old));
            segs.add_seg((i + 1, r, ch_old));
            segs.add_seg((i, i + 1, ch));

            ans.push(segs.max_len() as i32);
        }

        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("babacc", "bcb", vec![1,3,3], vec![3,3,4])]
    #[case("abyzz", "aa", vec![2,1], vec![2,3])]
    fn case(
        #[case] s: String,
        #[case] query_characters: String,
        #[case] query_indices: Vec<i32>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::longest_repeating(s, query_characters, query_indices);
        assert_eq!(actual, expected);
    }
}
