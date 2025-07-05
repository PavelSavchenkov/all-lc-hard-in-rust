//! Solution for https://leetcode.com/problems/orderly-queue
//! 899. Orderly Queue

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut s = to_u8(&s);
        let k = k as usize;
        let n = s.len();

        if k >= 2 {
            s.sort();
            return from_u8(&s);
        }

        s.extend(s.clone().iter());
        assert!(s.len() == n * 2);

        let mut min_i = 0;
        for i in 1..n {
            if s[i..i + n] < s[min_i..min_i + n] {
                min_i = i;
            }
        }
        from_u8(&s[min_i..min_i + n].to_vec())
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
    #[case("cba", 1, "acb")]
    #[case("baaca", 3, "aaabc")]
    fn case(#[case] s: String, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::orderly_queue(s, k);
        assert_eq!(actual, expected);
    }
}
