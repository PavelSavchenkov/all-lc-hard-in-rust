//! Solution for https://leetcode.com/problems/lexicographically-smallest-beautiful-string
//! 2663. Lexicographically Smallest Beautiful String

fn ok_to_append(s: &Vec<u8>, i: usize, ch: u8) -> bool {
    if i == 0 {
        return true;
    }
    if ch == s[i - 1] {
        return false;
    }
    if i == 1 {
        return true;
    }
    if ch == s[i - 2] {
        return false;
    }
    true
}

impl Solution {
    pub fn smallest_beautiful_string(s: String, k: i32) -> String {
        let mut s = to_u8(&s);
        let n = s.len();
        let k = k as usize;
        let k_ch = b'a' + k as u8;

        for i in (0..n).rev() {
            let lower = s[i] + 1;
            for nw in lower..k_ch {
                if ok_to_append(&s, i, nw) {
                    s[i] = nw;
                    for j in i + 1..n {
                        for ch in b'a'..k_ch {
                            if ok_to_append(&s, j, ch) {
                                s[j] = ch;
                                break;
                            }
                        }
                    }
                    return from_u8(&s);
                }
            }
        }

        String::new()
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
    #[case("abcz", 26, "abda")]
    #[case("dc", 4, "")]
    fn case(#[case] s: String, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::smallest_beautiful_string(s, k);
        assert_eq!(actual, expected);
    }
}
