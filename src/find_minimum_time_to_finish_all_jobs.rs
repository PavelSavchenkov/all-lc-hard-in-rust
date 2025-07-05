//! Solution for https://leetcode.com/problems/find-minimum-time-to-finish-all-jobs
//! 1723. Find Minimum Time to Finish All Jobs

fn remin(a: &mut i32, b: i32) {
    if *a > b {
        *a = b;
    }
}

impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = jobs.len();

        let mut sum = vec![0; 1 << n];
        for mask in 1..((1 << n) as usize) {
            let b = mask.trailing_zeros() as usize;
            sum[mask] = sum[mask ^ (1 << b)] + jobs[b];
        }

        let mut dp = vec![i32::MAX; 1 << n];
        dp[0] = 0;
        for it in 0..k {
            for mask in (0..((1 << n) as usize)).rev() {
                let cur_dp = dp[mask];
                let not_mask = !mask & ((1 << n) - 1);
                let mut sub = not_mask;
                while sub != 0 {
                    remin(&mut dp[mask | sub], cur_dp.max(sum[sub]));
                    sub = (sub - 1) & not_mask;
                }
            }
        }
        dp[(1 << n) - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,2,3], 3, 3)]
    #[case(vec![1,2,4,7,8], 2, 11)]
    fn case(#[case] jobs: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::minimum_time_required(jobs, k);
        assert_eq!(actual, expected);
    }
}
