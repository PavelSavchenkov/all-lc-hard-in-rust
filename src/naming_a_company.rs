//! Solution for https://leetcode.com/problems/naming-a-company
//! 2306. Naming a Company

use std::collections::HashSet;

const A: usize = 26;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let ideas = to_u8_vec(&ideas);

        let mut have = HashSet::new();
        for idea in &ideas {
            let h = get_hash(&idea);
            have.insert(h);
        }

        let mut cnt_ok = vec![vec![0; A]; A];
        for idea in &ideas {
            let first = (idea[0] - b'a') as usize;
            let mut swapped_idea = idea.clone();
            for another in 0..A {
                swapped_idea[0] = b'a' + another as u8;
                let h = get_hash(&swapped_idea);
                if !have.contains(&h) {
                    cnt_ok[first][another] += 1;
                }
            }
        }

        let mut ans = 0;
        for idea in &ideas {
            let first = (idea[0] - b'a') as usize;
            let mut swapped_idea = idea.clone();
            for another in 0..A {
                swapped_idea[0] = b'a' + another as u8;
                let h = get_hash(&swapped_idea);
                if have.contains(&h) {
                    continue;
                }
                ans += cnt_ok[another][first];
            }
        }

        ans
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
use std::collections::HashMap;
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
    #[case(vec!["coffee".into(),"donuts".into(),"time".into(),"toffee".into()], 6)]
    #[case(vec!["lack".into(),"back".into()], 0)]
    fn case(#[case] ideas: Vec<String>, #[case] expected: i64) {
        let actual = Solution::distinct_names(ideas);
        assert_eq!(actual, expected);
    }
}
