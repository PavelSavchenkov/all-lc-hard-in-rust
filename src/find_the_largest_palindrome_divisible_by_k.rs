//! Solution for https://leetcode.com/problems/find-the-largest-palindrome-divisible-by-k
//! 3260. Find the Largest Palindrome Divisible by K

impl Solution {
    pub fn largest_palindrome(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;

        let m = (n - 1) / 2;
        let mut pw10 = vec![0; n];
        pw10[0] = 1 % k;
        for i in 1..pw10.len() {
            pw10[i] = (pw10[i - 1] * 10) % k;
        }

        let mut dp = vec![vec![false; k]; m + 2];
        dp[m + 1][0] = true;
        for i in (0..=m).rev() {
            for rem in 0..k {
                for d in 0..10 {
                    let mut add = (pw10[i] * d) % k;
                    if i < n - 1 - i {
                        add = (add + pw10[n - 1 - i] * d) % k;
                    }
                    let prev_rem = (rem + k - add) % k;
                    if dp[i + 1][prev_rem] {
                        dp[i][rem] = true;
                    }
                }
            }
        }

        let mut ans = vec![0; n];
        let mut rem = 0;
        for i in 0..=m {
            for d in (0..10).rev() {
                let mut add = (pw10[i] * d) % k;
                if i < n - 1 - i {
                    add = (add + pw10[n - 1 - i] * d) % k;
                }
                let nrem = (rem + add) % k;
                if dp[i + 1][(k - nrem) % k] {
                    ans[i] = b'0' + d as u8;
                    ans[n - 1 - i] = b'0' + d as u8;
                    rem = nrem;
                    break;
                }
            }
        }

        String::from_utf8(ans).expect("")
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, 5, "595")]
    #[case(1, 4, "8")]
    #[case(5, 6, "89898")]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::largest_palindrome(n, k);
        assert_eq!(actual, expected);
    }
}
