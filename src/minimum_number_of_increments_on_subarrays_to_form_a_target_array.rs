//! Solution for https://leetcode.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array
//! 1526. Minimum Number of Increments on Subarrays to Form a Target Array

impl Solution {
    pub fn min_number_operations(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut ans = 0;
        let mut prev = 0;
        for &x in &a {
            if x > prev {
                ans += x - prev;
            }
            prev = x;
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
    #[case(vec![1,2,3,2,1], 3)]
    #[case(vec![3,1,1,2], 4)]
    #[case(vec![3,1,5,4,2], 7)]
    fn case(#[case] target: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_number_operations(target);
        assert_eq!(actual, expected);
    }
}
