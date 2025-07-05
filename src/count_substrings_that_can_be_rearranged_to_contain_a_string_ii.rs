//! Solution for https://leetcode.com/problems/count-substrings-that-can-be-rearranged-to-contain-a-string-ii
//! 3298. Count Substrings That Can Be Rearranged to Contain a String II

const A: usize = 26;

impl Solution {
    pub fn valid_substring_count(s: String, t: String) -> i64 {
        let s = to_u8(&s);
        let t = to_u8(&t);

        let n = s.len();

        let mut cnt_t = vec![0; A];
        for ch in &t {
            let c = (ch - b'a') as usize;
            cnt_t[c] += 1;
        }

        let mut positions = vec![Vec::new(); A];
        for i in 0..n {
            let c = (s[i] - b'a') as usize;
            positions[c].push(i);
        }

        let mut left = n;
        for c in 0..A {
            if cnt_t[c] > 0 {
                let len = positions[c].len();
                if len < cnt_t[c] {
                    return 0;
                }
                left = left.min(positions[c][len - cnt_t[c]]);
            }
        }

        let mut ans = 0;
        for i in (0..n).rev() {
            assert!(left <= i);
            ans += (left + 1) as i64;

            let c = (s[i] - b'a') as usize;
            if cnt_t[c] > 0 {
                positions[c].pop();
                let len = positions[c].len();
                if len < cnt_t[c] {
                    return ans;
                }
                left = left.min(positions[c][len - cnt_t[c]]);
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
    #[case("bcca", "abc", 1)]
    #[case("abcabc", "abc", 10)]
    #[case("abcabc", "aaabc", 0)]
    fn case(#[case] word1: String, #[case] word2: String, #[case] expected: i64) {
        let actual = Solution::valid_substring_count(word1, word2);
        assert_eq!(actual, expected);
    }
}
