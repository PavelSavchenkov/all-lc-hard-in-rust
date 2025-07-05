//! Solution for https://leetcode.com/problems/minimum-cost-to-divide-array-into-subarrays
//! 3500. Minimum Cost to Divide Array Into Subarrays

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, cost: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        assert!(cost.len() == n);
        let k = k as i64;

        let mut p_num = vec![0; n];
        p_num[0] = nums[0];
        for i in 1..n {
            p_num[i] = p_num[i - 1] + nums[i];
        }

        let mut p_cost = vec![0; n + 1];
        for i in 0..n {
            p_cost[i + 1] = p_cost[i] + cost[i];
        }

        let mut dp = vec![i64::MAX; n + 1];
        dp[n] = 0;
        for l in (0..n).rev() {
            for r in l..n {
                let mut cur = (p_num[r] as i64 + k) * (p_cost[r + 1] - p_cost[l]) as i64;
                cur += dp[r + 1];
                cur += k * (p_cost[n] - p_cost[r + 1]) as i64;
                dp[l] = dp[l].min(cur);
            }
        }
        dp[0]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,1,4], vec![4,6,6], 1, 110)]
    #[case(vec![4,8,5,1,14,2,2,12,1], vec![7,2,8,4,2,2,1,1,2], 7, 985)]
    fn case(#[case] nums: Vec<i32>, #[case] cost: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::minimum_cost(nums, cost, k);
        assert_eq!(actual, expected);
    }
}
