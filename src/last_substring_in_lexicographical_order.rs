//! Solution for https://leetcode.com/problems/last-substring-in-lexicographical-order
//! 1163. Last Substring in Lexicographical Order

impl Solution {
    pub fn last_substring(s: String) -> String {
        let mut s = to_u8(&s);
        s.push(0);
        let n = s.len();

        let mut cls = vec![0; 2 * n];
        let mut ord: Vec<_> = (0..n).collect();
        ord.sort_by_key(|&i| s[i]);
        let mut classes = 0;
        for j in 0..ord.len() {
            cls[ord[j]] = classes;
            if j + 1 < ord.len() {
                if s[ord[j]] < s[ord[j + 1]] {
                    classes += 1;
                }
            }
        }

        let mut log = 1;
        while (1 << log) < n {
            log += 1;
        }
        for k in 1..=log {
            for i in 0..n {
                cls[i + n] = cls[i];
            }

            let len = 1 << (k - 1);
            let mut ord: Vec<_> = (0..n).collect();
            ord.sort_by_key(|&i| (cls[i], cls[i + len]));
            let mut classes = 0;
            let mut ncls = vec![0; 2 * n];
            for j in 0..ord.len() {
                ncls[ord[j]] = classes;
                if j + 1 < ord.len() {
                    let cur = (cls[ord[j]], cls[ord[j] + len]);
                    let next = (cls[ord[j + 1]], cls[ord[j + 1] + len]);
                    assert!(cur <= next);
                    if cur < next {
                        classes += 1;
                    }
                }
            }
            cls = ncls;
        }

        let mx_cls = *cls.iter().max().unwrap();
        let pos = cls.iter().position(|&c| c == mx_cls).unwrap();
        s.pop();
        from_u8(&s[pos..].to_vec())
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
    #[case("abab", "bab")]
    #[case("leetcode", "tcode")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::last_substring(s);
        assert_eq!(actual, expected);
    }
}
