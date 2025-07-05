//! Solution for https://leetcode.com/problems/count-paths-that-can-form-a-palindrome-in-a-tree
//! 2791. Count Paths That Can Form a Palindrome in a Tree

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn count_palindrome_paths(parent: Vec<i32>, s: String) -> i64 {
        let s = to_u8(&s);
        let n = parent.len();
        let mut g = vec![Vec::new(); n];
        for i in 1..n {
            let p = parent[i] as usize;
            g[p].push(i);
        }

        let mut q = VecDeque::new();
        let mut masks = vec![0; n];
        let mut cnt_mask = HashMap::new();
        q.push_back(0);
        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            if v != 0 {
                masks[v] = masks[parent[v] as usize] ^ (1 << (s[v] - b'a') as usize);
            }
            *cnt_mask.entry(masks[v]).or_insert(0) += 1;
            for &to in &g[v] {
                q.push_back(to);
            }
        }

        let mut ans = 0;
        for v in 0..n {
            let cnt = cnt_mask.get(&masks[v]).unwrap() - 1;
            ans += cnt as i64;
            for c in 0..26 {
                let flip = 1 << c;
                let nmask = masks[v] ^ flip;
                let cnt = cnt_mask.get(&nmask);
                if cnt.is_some() {
                    ans += *cnt.unwrap() as i64;
                }
            }
        }
        ans /= 2;

        ans as i64
    }
}

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

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![-1,0,0,1,1,2], "acaabc", 8)]
    #[case(vec![-1,0,0,0,0], "aaaaa", 10)]
    fn case(#[case] parent: Vec<i32>, #[case] s: String, #[case] expected: i64) {
        let actual = Solution::count_palindrome_paths(parent, s);
        assert_eq!(actual, expected);
    }
}
