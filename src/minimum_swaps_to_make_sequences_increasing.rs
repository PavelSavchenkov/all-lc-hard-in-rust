//! Solution for https://leetcode.com/problems/minimum-swaps-to-make-sequences-increasing
//! 801. Minimum Swaps To Make Sequences Increasing

fn remin(a: &mut i32, b: i32) {
    if *a > b {
        *a = b;
    }
}

impl Solution {
    pub fn min_swap(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let n = a.len();
        assert!(b.len() == n);

        let mut dp = vec![vec![i32::MAX; 2]; n + 1];
        dp[1][0] = 0;
        dp[1][1] = 1;
        for i in 1..n {
            for swap in 0..2 {
                let cur_dp = dp[i][swap];
                assert!(cur_dp < i32::MAX);
                let mut prev = vec![a[i - 1], b[i - 1]];
                if swap == 1 {
                    prev.reverse();
                }
                for cur_swap in 0..2 {
                    let mut ndp = cur_dp;
                    let mut cur = vec![a[i], b[i]];
                    if cur_swap == 1 {
                        cur.reverse();
                        ndp += 1;
                    }
                    if !(prev[0] < cur[0] && prev[1] < cur[1]) {
                        continue;
                    }
                    remin(&mut dp[i + 1][cur_swap], ndp);
                }
            }
        }

        dp[n][0].min(dp[n][1])
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,5,4], vec![1,2,3,7], 1)]
    #[case(vec![0,3,5,8,9], vec![2,1,4,6,9], 1)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_swap(nums1, nums2);
        assert_eq!(actual, expected);
    }
}
