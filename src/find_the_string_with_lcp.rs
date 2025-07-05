//! Solution for https://leetcode.com/problems/find-the-string-with-lcp
//! 2573. Find the String with LCP

fn get_lcp(s: &Vec<u8>) -> Vec<Vec<i32>> {
    let n = s.len();
    let mut lcp = vec![vec![0; n]; n];
    for i in (0..n).rev() {
        for j in (0..n).rev() {
            if s[i] == s[j] {
                lcp[i][j] = 1;
                if i + 1 < n && j + 1 < n {
                    lcp[i][j] += lcp[i + 1][j + 1];
                }
            }
        }
    }
    lcp
}

impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();

        let mut s = vec![0 as u8; n];
        let mut ch = b'a' - 1;
        for i in 0..n {
            if s[i] != 0 {
                continue;
            }
            if ch == b'z' {
                return String::new();
            }
            ch += 1;
            for j in i..n {
                if lcp[i][j] >= 1 {
                    s[j] = ch;
                }
            }
        }

        let my_lcp = get_lcp(&s);
        if my_lcp != lcp {
            return String::new();
        }
        from_u8(&s)
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
    #[case(vec![vec![4,0,2,0],vec![0,3,0,1],vec![2,0,2,0],vec![0,1,0,1]], "abab")]
    #[case(vec![vec![4,3,2,1],vec![3,3,2,1],vec![2,2,2,1],vec![1,1,1,1]], "aaaa")]
    #[case(vec![vec![4,3,2,1],vec![3,3,2,1],vec![2,2,2,1],vec![1,1,1,3]], "")]
    fn case(#[case] lcp: Vec<Vec<i32>>, #[case] expected: String) {
        let actual = Solution::find_the_string(lcp);
        assert_eq!(actual, expected);
    }
}
