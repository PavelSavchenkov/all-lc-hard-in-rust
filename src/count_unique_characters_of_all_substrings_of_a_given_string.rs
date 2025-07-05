//! Solution for https://leetcode.com/problems/count-unique-characters-of-all-substrings-of-a-given-string
//! 828. Count Unique Characters of All Substrings of a Given String

const A: usize = 26;

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let s = to_u8(&s);
        let n = s.len();

        let mut next = vec![vec![n; A]; n + 1];
        for i in (0..n).rev() {
            for c in 0..A {
                next[i][c] = next[i + 1][c];
            }
            next[i][(s[i] - b'A') as usize] = i;
        }

        let mut ans = 0;
        for l in 0..n {
            let mut es = Vec::new();
            for c in 0..A {
                let nxt = next[l][c];
                es.push(nxt);
                if nxt < n {
                    es.push(next[nxt + 1][c]);
                }
            }
            es.push(n);
            es.sort();
            es.dedup();
            let mut cnt_unique = 0;
            for i in 0..es.len() - 1 {
                let pos = es[i];
                assert!(pos < n);
                let c = (s[pos] - b'A') as usize;
                let first = next[l][c];
                if pos == first {
                    cnt_unique += 1;
                } else if first < n && pos == next[first + 1][c] {
                    cnt_unique -= 1;
                }
                let coef = es[i + 1] - es[i];
                ans += cnt_unique * coef;
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

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ABC", 10)]
    #[case("ABA", 8)]
    #[case("LEETCODE", 92)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::unique_letter_string(s);
        assert_eq!(actual, expected);
    }
}
