//! Solution for https://leetcode.com/problems/longest-happy-prefix
//! 1392. Longest Happy Prefix

fn calc_prefix_function(s: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let mut p = vec![0; n];
    for i in 1..n {
        let mut j = p[i - 1];
        while j > 0 && s[i] != s[j] {
            j = p[j - 1];
        }
        if s[i] == s[j] {
            p[i] = j + 1;
        }
    }
    p
}

impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let s = to_u8(&s);
        let n = s.len();
        let p = calc_prefix_function(&s);
        let len = p[n - 1];
        from_u8(&s[..len].to_vec())
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
    #[case("level", "l")]
    #[case("ababab", "abab")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::longest_prefix(s);
        assert_eq!(actual, expected);
    }
}
