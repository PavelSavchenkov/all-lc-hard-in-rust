//! Solution for https://leetcode.com/problems/palindrome-partitioning-iii
//! 1278. Palindrome Partitioning III

impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let k = k as usize;
        let s = to_u8(&s);
        let n = s.len();

        let mut score = vec![vec![0; n]; n];
        for len in 1..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                if s[l] != s[r] {
                    score[l][r] += 1;
                }
                if l + 2 <= r {
                    score[l][r] += score[l + 1][r - 1];
                }
            }
        }

        let mut dp = vec![usize::MAX; n + 1];
        dp[0] = 0;
        for it in 0..k {
            let mut ndp = vec![usize::MAX; n + 1];
            for i in 0..=n {
                for j in 0..i {
                    if dp[j] == usize::MAX {
                        continue;
                    }
                    ndp[i] = ndp[i].min(dp[j] + score[j][i - 1]);
                }
            }
            dp = ndp;
        }
        let ans = dp[n];
        assert!(ans < usize::MAX);
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
    #[case("abc", 2, 1)]
    #[case("aabbc", 3, 0)]
    #[case("leetcode", 8, 0)]
    fn case(#[case] s: String, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::palindrome_partition(s, k);
        assert_eq!(actual, expected);
    }
}
