//! Solution for https://leetcode.com/problems/minimum-cost-to-cut-a-stick
//! 1547. Minimum Cost to Cut a Stick

impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        cuts.sort();

        let mut a = Vec::new();
        let mut prev = 0;
        for i in 0..cuts.len() {
            let len = cuts[i] - prev;
            a.push(len);
            prev = cuts[i];
        }
        a.push(n - cuts.last().unwrap());

        let mut pref = vec![0; a.len() + 1];
        for i in 0..a.len() {
            pref[i + 1] = pref[i] + a[i];
        }

        let mut dp = vec![vec![i32::MAX; a.len()]; a.len()];
        for len in 1..=a.len() {
            for l in 0..=a.len() - len {
                let r = l + len - 1;
                if len == 1 {
                    dp[l][r] = 0;
                } else {
                    for m in l..r {
                        let cost = pref[r + 1] - pref[l];
                        dp[l][r] = dp[l][r].min(dp[l][m] + dp[m + 1][r] + cost);
                    }
                }
            }
        }
        dp[0][a.len() - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(7, vec![1,3,4,5], 16)]
    #[case(9, vec![5,6,1,4,2], 22)]
    fn case(#[case] n: i32, #[case] cuts: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_cost(n, cuts);
        assert_eq!(actual, expected);
    }
}
