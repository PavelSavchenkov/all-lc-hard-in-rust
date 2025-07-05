//! Solution for https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule
//! 1335. Minimum Difficulty of a Job Schedule

impl Solution {
    pub fn min_difficulty(a: Vec<i32>, d: i32) -> i32 {
        let n = a.len();
        let d = d as usize;

        if d > n {
            return -1;
        }

        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        for it in 0..d {
            let mut ndp = vec![i32::MAX; n + 1];
            for i in 0..n {
                let cur_dp = dp[i];
                if cur_dp == i32::MAX {
                    continue;
                }
                let mut mx = a[i];
                for j in i..n {
                    mx = mx.max(a[j]);
                    ndp[j + 1] = ndp[j + 1].min(cur_dp + mx);
                }
            }
            dp = ndp;
        }
        dp[n]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![6,5,4,3,2,1], 2, 7)]
    #[case(vec![9,9,9], 4, -1)]
    #[case(vec![1,1,1], 3, 3)]
    fn case(#[case] job_difficulty: Vec<i32>, #[case] d: i32, #[case] expected: i32) {
        let actual = Solution::min_difficulty(job_difficulty, d);
        assert_eq!(actual, expected);
    }
}
