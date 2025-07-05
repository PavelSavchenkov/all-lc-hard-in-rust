//! Solution for https://leetcode.com/problems/minimum-number-of-moves-to-make-palindrome
//! 2193. Minimum Number of Moves to Make Palindrome

const A: usize = 26;

impl Solution {
    pub fn min_moves_to_make_palindrome(s: String) -> i32 {
        let mut s = to_u8(&s);

        let mut cnt = vec![0; A];
        for &ch in &s {
            let c = (ch - b'a') as usize;
            cnt[c] += 1;
        }
        let mut odd_ch = 0;
        for c in 0..A {
            if cnt[c] % 2 == 1 {
                assert!(odd_ch == 0);
                odd_ch = b'a' + c as u8;
            }
        }

        let mut ans = 0;
        while s.len() > 1 {
            let ch = *s.last().unwrap();
            let c = (ch - b'a') as usize;
            if cnt[c] == 1 && ch == odd_ch {
                let len = s.len();
                s.swap(len - 2, len - 1);
                ans += 1;
                continue;
            }
            cnt[c] -= 1;
            s.pop();
            let i = s.iter().position(|&c| c == ch).unwrap();
            ans += i;
            let ch = s.remove(i);
            let c = (ch - b'a') as usize;
            cnt[c] -= 1;
        }
        ans as i32
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
    #[case("aabb", 2)]
    #[case("letelt", 2)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::min_moves_to_make_palindrome(s);
        assert_eq!(actual, expected);
    }
}
