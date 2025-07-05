//! Solution for https://leetcode.com/problems/count-beautiful-substrings-ii
//! 2949. Count Beautiful Substrings II

use std::collections::HashMap;

fn is_vowel(ch: u8) -> bool {
    "aeiou".contains(ch as char)
}

impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i64 {
        let s = to_u8(&s);
        let n = s.len();
        let k = k as usize;

        let mut good_rems = Vec::new();
        for rem in 0..4 * k {
            if (rem * rem) % (4 * k) == 0 {
                good_rems.push(rem);
            }
        }

        let mut pref = vec![0 as i32; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i];
            if is_vowel(s[i]) {
                pref[i + 1] += 1;
            }
        }

        let mut ans = 0;
        let mut cnt = vec![vec![HashMap::new(); 2]; 4 * k];
        for R in 1..=n {
            {
                let L = R - 1;
                *cnt[L % (4 * k)][L % 2]
                    .entry(pref[L] - L as i32 / 2)
                    .or_insert(0) += 1;
            }
            for &rem in &good_rems {
                let L_rem = (R % (4 * k) + (4 * k) - rem) % (4 * k);
                ans += *cnt[L_rem][R % 2].entry(pref[R] - R as i32 / 2).or_default();
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

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("baeyh", 2, 2)]
    #[case("abba", 1, 3)]
    #[case("bcdf", 1, 0)]
    fn case(#[case] s: String, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::beautiful_substrings(s, k);
        assert_eq!(actual, expected);
    }
}
