//! Solution for https://leetcode.com/problems/find-the-closest-palindrome
//! 564. Find the Closest Palindrome

fn make_pal(mut x: i64, len: usize) -> i64 {
    let mut left = x;
    if len % 2 == 1 {
        left /= 10;
    }
    while left > 0 {
        x = x * 10 + left % 10;
        left /= 10;
    }
    x
}

impl Solution {
    pub fn nearest_palindromic(digits: String) -> String {
        let n: i64 = digits.parse().expect("Expect valid number");
        let digits = to_u8(&digits);
        let len = digits.len();

        if n <= 9 {
            return (n - 1).to_string();
        }

        let mut candidates = Vec::new();

        {
            let next = i64::pow(10, len as u32) + 1;
            candidates.push(next);
        }

        {
            let prev = i64::pow(10, (len - 1) as u32) - 1;
            candidates.push(prev);
        }

        let half: i64 = from_u8(&digits[..(len + 1) / 2].to_vec())
            .parse()
            .expect("");
        assert!(half > 0);
        for dx in [-1, 0, 1] {
            let new_half = half + dx;
            let new_cand = make_pal(new_half, len);
            candidates.push(new_cand);
        }

        let mut best_cand = candidates[0];
        let mut best_diff = (n - best_cand).abs();
        for &cand in &candidates {
            if cand == n {
                continue;
            }
            let diff = (n - cand).abs();
            if diff < best_diff {
                best_cand = cand;
                best_diff = diff;
            } else if diff == best_diff && cand < best_cand {
                best_cand = cand;
            }
        }

        best_cand.to_string()
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
    #[case("123", "121")]
    #[case("1", "0")]
    fn case(#[case] n: String, #[case] expected: String) {
        let actual = Solution::nearest_palindromic(n);
        assert_eq!(actual, expected);
    }
}
