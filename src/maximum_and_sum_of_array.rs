//! Solution for https://leetcode.com/problems/maximum-and-sum-of-array
//! 2172. Maximum AND Sum of Array

impl Solution {
    pub fn maximum_and_sum(a: Vec<i32>, num_slots: i32) -> i32 {
        let num_slots = num_slots as usize;
        let n = a.len();

        let mut slots = Vec::new();
        for i in 1..=num_slots {
            slots.extend(vec![i as i32; 2]);
        }

        let mut dp = vec![-1; 1 << n];
        dp[0] = 0;
        for i in 0..slots.len() {
            for mask in (0..1 << n).rev() {
                if dp[mask] == -1 {
                    continue;
                }
                for j in 0..n {
                    if ((mask >> j) & 1) == 0 {
                        let nmask = mask ^ (1 << j);
                        dp[nmask] = dp[nmask].max(dp[mask] + (slots[i] & a[j]));
                    }
                }
            }
        }

        *dp.iter().max().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4,5,6], 3, 9)]
    #[case(vec![1,3,10,4,7,1], 9, 24)]
    fn case(#[case] nums: Vec<i32>, #[case] num_slots: i32, #[case] expected: i32) {
        let actual = Solution::maximum_and_sum(nums, num_slots);
        assert_eq!(actual, expected);
    }
}
