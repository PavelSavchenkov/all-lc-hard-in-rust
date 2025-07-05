//! Solution for https://leetcode.com/problems/maximum-score-of-spliced-array
//! 2321. Maximum Score Of Spliced Array

impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let a = vec![nums1, nums2];

        let mut pref = vec![vec![0; n + 1]; 2];
        for it in 0..2 {
            for i in 0..n {
                pref[it][i + 1] = pref[it][i] + a[it][i];
            }
        }

        let mut ans = 0;
        for it in 0..2 {
            let mut mx = 0;
            for i in 0..n {
                let suff_sum = pref[it][n] - pref[it][i + 1];
                let pref_sum = pref[it ^ 1][i + 1] + mx;
                ans = ans.max(pref_sum + suff_sum);
                mx = mx.max(-pref[it ^ 1][i + 1] + pref[it][i + 1]);
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
    #[case(vec![60,60,60], vec![10,90,10], 210)]
    #[case(vec![20,40,20,70,30], vec![50,20,50,40,20], 220)]
    #[case(vec![7,11,13], vec![1,1,1], 31)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::maximums_spliced_array(nums1, nums2);
        assert_eq!(actual, expected);
    }
}
