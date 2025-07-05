//! Solution for https://leetcode.com/problems/maximum-profit-in-job-scheduling
//! 1235. Maximum Profit in Job Scheduling

struct Job {
    start_time: i32,
    end_time: i32,
    profit: i32,
}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = Vec::new();
        let n = start_time.len();
        for i in 0..n {
            jobs.push(Job {
                start_time: start_time[i],
                end_time: end_time[i],
                profit: profit[i],
            });
        }
        jobs.sort_by_key(|j| j.end_time);

        let mut dp = vec![0; n];
        for i in 0..n {
            dp[i] = jobs[i].profit;
            if i > 0 {
                dp[i] = dp[i].max(dp[i - 1]);
            }
            let j = jobs.partition_point(|j| j.end_time <= jobs[i].start_time);
            if j > 0 {
                dp[i] = dp[i].max(dp[j - 1] + jobs[i].profit);
            }
        }
        dp[n - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,3], vec![3,4,5,6], vec![50,10,40,70], 120)]
    #[case(vec![1,2,3,4,6], vec![3,5,10,6,9], vec![20,20,100,70,60], 150)]
    #[case(vec![1,1,1], vec![2,3,4], vec![5,6,4], 6)]
    fn case(
        #[case] start_time: Vec<i32>,
        #[case] end_time: Vec<i32>,
        #[case] profit: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::job_scheduling(start_time, end_time, profit);
        assert_eq!(actual, expected);
    }
}
