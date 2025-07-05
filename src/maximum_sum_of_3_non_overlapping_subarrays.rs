//! Solution for https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays
//! 689. Maximum Sum of 3 Non-Overlapping Subarrays

impl Solution {
    pub fn max_sum_of_three_subarrays(a: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = a.len();

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + a[i];
        }

        let mut dp = vec![vec![0; 4]; n + 1];
        for cnt in 1..=3 {
            for i in (0..=n - k * cnt).rev() {
                let mut ndp = dp[i + 1][cnt];
                let sum = pref[i + k] - pref[i];
                ndp = ndp.max(sum + dp[i + k][cnt - 1]);
                dp[i][cnt] = ndp;
            }
        }

        let mut i = 0;
        let mut cnt = 3;
        let mut positions = Vec::new();
        while cnt > 0 {
            let sum = pref[i + k] - pref[i];
            if dp[i][cnt] == sum + dp[i + k][cnt - 1] {
                positions.push(i as i32);
                i = i + k;
                cnt -= 1;
            } else {
                i += 1;
            }
        }

        positions
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,1,2,6,7,5,1], 2, vec![0,3,5])]
    #[case(vec![1,2,1,2,1,2,1,2,1], 2, vec![0,2,4])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::max_sum_of_three_subarrays(nums, k);
        assert_eq!(actual, expected);
    }
}
