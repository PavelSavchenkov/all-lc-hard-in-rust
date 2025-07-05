//! Solution for https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous
//! 2009. Minimum Number of Operations to Make Array Continuous

impl Solution {
    pub fn min_operations(mut a: Vec<i32>) -> i32 {
        a.sort();
        let n = a.len();
        a.dedup();

        let mut ans = n;
        let mut j = 0;
        for i in 0..a.len() {
            j = j.max(i + 1);
            while j < a.len() && a[j] <= a[i] + n as i32 - 1 {
                j += 1;
            }
            let have = j - i;
            ans = ans.min(n - have);
        }

        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![4,2,5,3], 0)]
    #[case(vec![1,2,3,5,6], 1)]
    #[case(vec![1,10,100,1000], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_operations(nums);
        assert_eq!(actual, expected);
    }
}
