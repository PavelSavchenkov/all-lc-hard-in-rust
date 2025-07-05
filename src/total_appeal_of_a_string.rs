//! Solution for https://leetcode.com/problems/total-appeal-of-a-string
//! 2262. Total Appeal of A String

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

const A: usize = 26;

impl Solution {
    pub fn appeal_sum(s: String) -> i64 {
        let s = to_u8(&s);
        let n = s.len();

        let mut ans: i64 = 0;
        let mut last = vec![n; A];
        for i in (0..n).rev() {
            last[(s[i] - b'a') as usize] = i;

            let mut ord: Vec<_> = (0..A).collect();
            ord.sort_by_key(|&c| last[c]);

            for j in 0..A {
                let start_len = last[ord[j]] - i + 1;
                let next = if j + 1 == A { n } else { last[ord[j + 1]] };
                let end_len = next - i + 1;
                ans += (end_len - start_len) as i64 * (j + 1) as i64;
            }
        }

        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abbca", 28)]
    #[case("code", 20)]
    fn case(#[case] s: String, #[case] expected: i64) {
        let actual = Solution::appeal_sum(s);
        assert_eq!(actual, expected);
    }
}
