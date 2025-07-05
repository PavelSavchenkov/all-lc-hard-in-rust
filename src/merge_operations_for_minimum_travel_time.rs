//! Solution for https://leetcode.com/problems/merge-operations-for-minimum-travel-time
//! 3538. Merge Operations for Minimum Travel Time

impl Solution {
    pub fn min_travel_time(l: i32, n: i32, k: i32, position: Vec<i32>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let k = k as usize;

        let mut pref_time = vec![0; n + 1];
        for i in 0..n {
            pref_time[i + 1] = pref_time[i] + time[i];
        }

        // dp[merges][j][i] -- j < i, last 2 kept signs
        let mut dp = vec![vec![vec![i32::MAX; n]; n]; k + 1];
        for merges in 0..=k {
            for j in 0..n {
                for i in j + 1..n {
                    let last_merges = i - j - 1;
                    if last_merges > merges {
                        continue;
                    }
                    if j == 0 {
                        if merges == last_merges {
                            dp[merges][j][i] = (position[i] - position[0]) * time[0];
                        }
                    } else {
                        let mut mn = i32::MAX;
                        for z in 0..j {
                            let prev_merges = merges - last_merges;
                            let prev_dp = dp[prev_merges][z][j];
                            if prev_dp == i32::MAX {
                                continue;
                            }
                            let mut cur = prev_dp;
                            cur +=
                                (pref_time[j + 1] - pref_time[z + 1]) * (position[i] - position[j]);
                            mn = mn.min(cur);
                        }
                        dp[merges][j][i] = mn;
                    }
                }
            }
        }

        let mut ans = i32::MAX;
        for j in 0..n - 1 {
            ans = ans.min(dp[k][j][n - 1]);
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
    #[case(10, 4, 1, vec![0,3,8,10], vec![5,8,3,6], 62)]
    #[case(5, 5, 1, vec![0,1,2,3,5], vec![8,3,9,3,3], 34)]
    fn case(
        #[case] l: i32,
        #[case] n: i32,
        #[case] k: i32,
        #[case] position: Vec<i32>,
        #[case] time: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::min_travel_time(l, n, k, position, time);
        assert_eq!(actual, expected);
    }
}
