//! Solution for https://leetcode.com/problems/check-if-string-is-transformable-with-substring-sort-operations
//! 1585. Check If String Is Transformable With Substring Sort Operations

impl Solution {
    pub fn is_transformable(s: String, t: String) -> bool {
        let s = to_u8(&s);
        let t = to_u8(&t);

        let mut poses = vec![Vec::new(); 10];
        for i in 0..s.len() {
            let c = (s[i] - b'0') as usize;
            poses[c].push(i);
        }

        let mut ptr = vec![0; 10];
        for &ch in &t {
            let c = (ch - b'0') as usize;
            if ptr[c] == poses[c].len() {
                return false;
            }
            for cc in 0..c {
                if ptr[cc] < poses[cc].len() && poses[cc][ptr[cc]] < poses[c][ptr[c]] {
                    return false;
                }
            }
            ptr[c] += 1;
        }
        true
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
    #[case("84532", "34852", true)]
    #[case("34521", "23415", true)]
    #[case("12345", "12435", false)]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: bool) {
        let actual = Solution::is_transformable(s, t);
        assert_eq!(actual, expected);
    }
}
