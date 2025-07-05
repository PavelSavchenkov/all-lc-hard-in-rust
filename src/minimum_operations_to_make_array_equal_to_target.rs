//! Solution for https://leetcode.com/problems/minimum-operations-to-make-array-equal-to-target
//! 3229. Minimum Operations to Make Array Equal to Target

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let n = nums.len();

        let mut a = Vec::new();
        for i in 0..n {
            a.push(nums[i] - target[i]);
        }

        let mut ans = 0;
        let mut prev: i32 = 0;
        for i in 0..n {
            if a[i] != 0 {
                if a[i].signum() != prev.signum() {
                    prev = 0;
                }
                if a[i] > 0 {
                    if a[i] > prev {
                        ans += (a[i] - prev) as i64;
                    }
                } else {
                    assert!(a[i] < 0);
                    if a[i] < prev {
                        ans += (prev - a[i]) as i64;
                    }
                }
            }
            prev = a[i];
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
    #[case(vec![3,5,1,2], vec![4,6,2,4], 2)]
    #[case(vec![1,3,2], vec![2,1,4], 5)]
    fn case(#[case] nums: Vec<i32>, #[case] target: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::minimum_operations(nums, target);
        assert_eq!(actual, expected);
    }
}
