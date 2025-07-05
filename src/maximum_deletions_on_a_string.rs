//! Solution for https://leetcode.com/problems/maximum-deletions-on-a-string
//! 2430. Maximum Deletions on a String

fn calc_z(s: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let mut l = 0;
    let mut r = 0;
    let mut z = vec![0; n];
    for i in 1..n {
        if i < r {
            z[i] = (r - i).min(z[i - l]);
        }
        while i + z[i] < n && s[i + z[i]] == s[z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z[0] = n;
    z
}

impl Solution {
    pub fn delete_string(s: String) -> i32 {
        let s = to_u8(&s);
        let n = s.len();

        let mut dp = vec![1; n + 1];
        for i in (0..n).rev() {
            let z = calc_z(&s[i..].to_vec());
            for j in i + 1..n {
                if z[j - i] >= j - i {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }

        dp[0] as i32
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
    #[case("abcabcdabc", 2)]
    #[case("aaabaab", 4)]
    #[case("aaaaa", 5)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::delete_string(s);
        assert_eq!(actual, expected);
    }
}
