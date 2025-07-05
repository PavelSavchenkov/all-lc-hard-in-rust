//! Solution for https://leetcode.com/problems/smallest-k-length-subsequence-with-occurrences-of-a-letter
//! 2030. Smallest K-Length Subsequence With Occurrences of a Letter

const A: usize = 26;

impl Solution {
    pub fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        let s: Vec<u8> = s.as_bytes().iter().map(|&c| (c - b'a')).collect();
        let k = k as usize;
        let letter = letter as u8 - b'a';
        let mut repetition = repetition as usize;
        let n = s.len();

        let mut suff = vec![0 as usize; n + 1];
        let mut next = vec![[n; A]; n + 1];
        for i in (0..n).rev() {
            suff[i] = suff[i + 1];
            if s[i] == letter {
                suff[i] += 1;
            }
            for c in 0..A {
                next[i][c] = next[i + 1][c];
            }
            next[i][s[i] as usize] = i;
        }
        assert!(suff[0] >= repetition);

        let mut ans = Vec::new();
        let mut i = 0;
        while ans.len() < k {
            let mut found = false;
            for c in 0..A {
                let j = next[i][c];
                if suff[j] < repetition {
                    continue
                }
                if ans.len() + (n - j as usize) < k {
                    continue
                }
                if ans.len() + repetition == k {
                    if c as u8 != letter {
                        continue
                    }
                }
                ans.push(c as u8);
                if c as u8 == letter && repetition > 0 {
                    repetition -= 1;
                }
                i = j + 1;
                found = true;
                break;
            }
            assert!(found);
            if i >= n {
                break
            }
        };
        assert!(ans.len() == k);

        String::from_utf8(ans.iter().map(|c| b'a' + c).collect()).unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("leet", 3, 'e', 1, "eet")]
    #[case("leetcode", 4, 'e', 2, "ecde")]
    #[case("bb", 2, 'b', 2, "bb")]
    #[case("mmmxmxymmm", 8, 'm', 4, "mmmmxmmm")]
    fn case(
        #[case] s: String,
        #[case] k: i32,
        #[case] letter: char,
        #[case] repetition: i32,
        #[case] expected: String,
    ) {
        let actual = Solution::smallest_subsequence(s, k, letter, repetition);
        assert_eq!(actual, expected);
    }
}
