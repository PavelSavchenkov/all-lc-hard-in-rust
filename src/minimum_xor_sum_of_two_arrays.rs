//! Solution for https://leetcode.com/problems/minimum-xor-sum-of-two-arrays
//! 1879. Minimum XOR Sum of Two Arrays

impl Solution {
    pub fn minimum_xor_sum(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let n = a.len();

        let mut dp = vec![i32::MAX; 1 << n];
        dp[0] = 0;
        for mask in 1..((1 << n) as usize) {
            let last = mask.count_ones() as usize - 1;
            for i in 0..n {
                if ((mask >> i) & 1) == 1 {
                    let ndp = dp[mask ^ (1 << i)] + (a[last] ^ b[i]);
                    dp[mask] = dp[mask].min(ndp);
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
    #[case(vec![1,2], vec![2,3], 2)]
    #[case(vec![1,0,3], vec![5,3,4], 8)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::minimum_xor_sum(nums1, nums2);
        assert_eq!(actual, expected);
    }
}
