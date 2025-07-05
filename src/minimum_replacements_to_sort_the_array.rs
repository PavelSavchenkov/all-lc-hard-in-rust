//! Solution for https://leetcode.com/problems/minimum-replacements-to-sort-the-array
//! 2366. Minimum Replacements to Sort the Array

impl Solution {
    pub fn minimum_replacement(a: Vec<i32>) -> i64 {
        let n = a.len();
        let mut ans: i64 = 0;
        let mut mn = i32::MAX;
        for &x in a.iter().rev() {
            if x > mn {
                let slots = (x + mn - 1) / mn;
                assert!(slots > 1);
                ans += (slots - 1) as i64;
                mn = x / slots;
            } else {
                mn = x;
            }
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
    #[case(vec![3,9,3], 2)]
    #[case(vec![1,2,3,4,5], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::minimum_replacement(nums);
        assert_eq!(actual, expected);
    }
}
