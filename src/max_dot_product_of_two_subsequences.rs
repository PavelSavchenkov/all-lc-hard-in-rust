//! Solution for https://leetcode.com/problems/max-dot-product-of-two-subsequences
//! 1458. Max Dot Product of Two Subsequences

fn remax(a: &mut i32, b: i32) {
    if *a < b {
        *a = b;
    }
}

impl Solution {
    pub fn max_dot_product(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let n = a.len();
        let m = b.len();

        let mut dp = vec![vec![i32::MIN; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                dp[i + 1][j + 1] = a[i] * b[j];
            }
        }
        for i in 0..=n {
            for j in 0..=m {
                let cur_dp = dp[i][j];
                if cur_dp == i32::MIN {
                    continue;
                }
                if i + 1 <= n {
                    remax(&mut dp[i + 1][j], cur_dp);
                }
                if j + 1 <= m {
                    remax(&mut dp[i][j + 1], cur_dp);
                }
                if i + 1 <= n && j + 1 <= m {
                    remax(&mut dp[i + 1][j + 1], cur_dp + a[i] * b[j]);
                }
            }
        }
        dp[n][m]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,1,-2,5], vec![3,0,-6], 18)]
    #[case(vec![3,-2], vec![2,-6,7], 21)]
    #[case(vec![-1,-1], vec![1,1], -1)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_dot_product(nums1, nums2);
        assert_eq!(actual, expected);
    }
}
