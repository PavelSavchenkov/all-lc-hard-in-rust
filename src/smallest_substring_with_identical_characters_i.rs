//! Solution for https://leetcode.com/problems/smallest-substring-with-identical-characters-i
//! 3398. Smallest Substring With Identical Characters I

impl Solution {
    pub fn min_length(s: String, num_ops: i32) -> i32 {
        let s = to_u8(&s);
        let n = s.len();
        let num_ops = num_ops as usize;

        let need_ops = |max_len: usize| -> usize {
            if max_len == 0 {
                return usize::MAX;
            }
            if max_len == 1 {
                let mut ans = usize::MAX;
                for it in 0..2 {
                    let mut cnt = 0;
                    for i in 0..n {
                        let c = s[i] - b'0';
                        if c != ((i % 2) as u8 + it) % 2 {
                            cnt += 1;
                        }
                    }
                    ans = ans.min(cnt);
                }
                return ans;
            }
            assert!(max_len >= 2);
            let mut cnt = 0;
            let mut len = 1;
            let mut prev = s[0] - b'0';
            for i in 1..n {
                let c = s[i] - b'0';
                if c == prev {
                    len += 1;
                    if len == max_len + 1 {
                        cnt += 1;
                        len = 1;
                        prev = 2;
                    }
                } else {
                    len = 1;
                    prev = c;
                }
            }
            cnt
        };

        let mut L = 0;
        let mut R = n + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if need_ops(M) <= num_ops {
                R = M;
            } else {
                L = M;
            }
        }
        R as i32
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
    #[case("000001", 1, 2)]
    #[case("0000", 2, 1)]
    #[case("0101", 0, 1)]
    fn case(#[case] s: String, #[case] num_ops: i32, #[case] expected: i32) {
        let actual = Solution::min_length(s, num_ops);
        assert_eq!(actual, expected);
    }
}
