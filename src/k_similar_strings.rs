//! Solution for https://leetcode.com/problems/k-similar-strings
//! 854. K-Similar Strings

use std::collections::HashMap;
use std::collections::VecDeque;

fn state(mask: usize, a: usize, b: usize) -> u64 {
    get_hash(&vec![mask, a, b])
}

impl Solution {
    pub fn k_similarity(s1: String, s2: String) -> i32 {
        let s1 = to_u8(&s1);
        let s2 = to_u8(&s2);
        let n = s1.len();

        let mut g = vec![vec![false; n]; n];
        for i in 0..n {
            for j in 0..n {
                if s1[i] == s2[j] {
                    g[i][j] = true;
                }
            }
        }

        let mut dp = HashMap::<u64, u8>::new();
        let mut q = VecDeque::new();
        for i in 0..n {
            let mask = 1 << i;
            dp.insert(state(mask, i, i), 1);
            q.push_back((mask, i, i, 1));
        }
        let full_mask = (1 << n) - 1;
        let mut ans = 0;
        while !q.is_empty() {
            let (mask, a, b, cur_dp) = q.pop_front().unwrap();
            if mask == full_mask {
                ans = ans.max(cur_dp as usize);
                continue;
            }

            let mut was_new_cycle = false;
            for c in 0..n {
                if ((mask >> c) & 1) == 1 {
                    continue;
                }
                let nmask = mask | 1 << c;
                let ndp;
                let nb;
                if g[a][b] {
                    // start new cycle
                    if was_new_cycle {
                        continue;
                    }
                    was_new_cycle = true;
                    ndp = cur_dp + 1;
                    nb = c;
                } else if g[a][c] {
                    // continue current cycle
                    ndp = cur_dp;
                    nb = b;
                } else {
                    continue;
                };
                let nstate = state(nmask, c, nb);
                let dp_nstate = dp.entry(nstate).or_insert(0);
                if *dp_nstate < ndp {
                    *dp_nstate = ndp;
                    q.push_back((nmask, c, nb, ndp));
                }
            }
        }
        (n - ans) as i32
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
    #[case("ab", "ba", 1)]
    #[case("abc", "bca", 2)]
    fn case(#[case] s1: String, #[case] s2: String, #[case] expected: i32) {
        let actual = Solution::k_similarity(s1, s2);
        assert_eq!(actual, expected);
    }
}
