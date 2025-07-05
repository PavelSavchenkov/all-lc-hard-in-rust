//! Solution for https://leetcode.com/problems/find-longest-awesome-substring
//! 1542. Find Longest Awesome Substring

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

const A: usize = 10;

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let s = to_u8(&s);
        let n = s.len();

        let mut ans = 0;
        let mut first = vec![n; 1 << A];
        let mut mask = 0;
        for i in 0..n {
            if first[mask] == n {
                first[mask] = i;
            }

            mask ^= 1 << ((s[i] - b'0') as usize);

            for c in 0..=A {
                let need_mask = if c == A { mask } else { mask ^ (1 << c) };
                let l = first[need_mask];
                if l < n {
                    let len = i - l + 1;
                    ans = ans.max(len);
                }
            }
        }
        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("3242415", 5)]
    #[case("12345678", 1)]
    #[case("213123", 6)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::longest_awesome(s);
        assert_eq!(actual, expected);
    }
}
