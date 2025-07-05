//! Solution for https://leetcode.com/problems/length-of-the-longest-valid-substring
//! 2781. Length of the Longest Valid Substring

use std::collections::HashSet;

impl Solution {
    pub fn longest_valid_substring(word: String, forbidden: Vec<String>) -> i32 {
        let word = to_u8(&word);

        let mut set = HashSet::new();
        let mut max_forbidden_len = 0;
        for s in &forbidden {
            let s = to_u8(&s);
            set.insert(get_hash(&s));
            max_forbidden_len = max_forbidden_len.max(s.len());
        }

        let mut ans = 0;
        let mut min_left = 0;
        for r in 0..word.len() {
            for i in (0..=r).rev() {
                let len = r - i + 1;
                if len > max_forbidden_len {
                    break;
                }
                let h = get_hash(&word[i..r + 1].to_vec());
                if set.contains(&h) {
                    min_left = min_left.max(i + 1);
                }
            }

            if min_left <= r {
                ans = ans.max(r - min_left + 1);
            }
        }

        ans as i32
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

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("cbaaaabc", vec!["aaa".into(),"cb".into()], 4)]
    #[case("leetcode", vec!["de".into(),"le".into(),"e".into()], 4)]
    fn case(#[case] word: String, #[case] forbidden: Vec<String>, #[case] expected: i32) {
        let actual = Solution::longest_valid_substring(word, forbidden);
        assert_eq!(actual, expected);
    }
}
