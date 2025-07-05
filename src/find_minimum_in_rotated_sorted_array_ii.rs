//! Solution for https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii
//! 154. Find Minimum in Rotated Sorted Array II

// impossible to do faster if there are duplicates
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        *nums.iter().min().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,5], 1)]
    #[case(vec![2,2,2,0,1], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::find_min(nums);
        assert_eq!(actual, expected);
    }
}
