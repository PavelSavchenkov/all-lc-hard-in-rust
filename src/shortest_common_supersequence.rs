//! Solution for https://leetcode.com/problems/shortest-common-supersequence
//! 1092. Shortest Common Supersequence

impl Solution {
    pub fn shortest_common_supersequence(s: String, t: String) -> String {
        let s = to_u8(&s);
        let t = to_u8(&t);
        let n = s.len();
        let m = t.len();

        let mut dp = vec![vec![usize::MAX; m + 1]; n + 1];
        let mut prev = vec![vec![(0, 0, 0); m + 1]; n + 1];
        dp[0][0] = 0;
        for i in 0..=n {
            for j in 0..=m {
                let cur_dp = dp[i][j];
                if cur_dp == usize::MAX {
                    continue;
                }
                let mut next = Vec::new();
                if i < n {
                    next.push(s[i]);
                }
                if j < m {
                    next.push(t[j]);
                }
                for &ch in &next {
                    let mut ni = i;
                    if i < n && s[i] == ch {
                        ni += 1;
                    }
                    let mut nj = j;
                    if j < m && t[j] == ch {
                        nj += 1;
                    }
                    if cur_dp + 1 < dp[ni][nj] {
                        prev[ni][nj] = (i, j, ch);
                        dp[ni][nj] = cur_dp + 1;
                    }
                }
            }
        }

        let mut ans = String::new();
        let mut i = n;
        let mut j = m;
        while i > 0 || j > 0 {
            let (pi, pj, ch) = prev[i][j];
            ans.push(ch as char);
            i = pi;
            j = pj;
        }
        ans = ans.chars().rev().collect();
        ans
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
    #[case("abac", "cab", "cabac")]
    #[case("aaaaaaaa", "aaaaaaaa", "aaaaaaaa")]
    fn case(#[case] str1: String, #[case] str2: String, #[case] expected: String) {
        let actual = Solution::shortest_common_supersequence(str1, str2);
        assert_eq!(actual, expected);
    }
}
