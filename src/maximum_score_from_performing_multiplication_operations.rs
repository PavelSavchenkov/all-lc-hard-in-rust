//! Solution for https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations
//! 1770. Maximum Score from Performing Multiplication Operations

fn remax(a: &mut i32, b: i32) {
    if *a < b {
        *a = b;
    }
}

impl Solution {
    pub fn maximum_score(a: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = a.len();
        let m = multipliers.len();

        let mut dp = vec![i32::MIN; m + 1];
        dp[0] = 0;
        for i in 0..m {
            let mut ndp = vec![i32::MIN; m + 1];
            for cnt_left in 0..=i {
                let cnt_right = i - cnt_left;
                let cur_dp = dp[cnt_left];
                if cur_dp == i32::MIN {
                    continue;
                }
                // take from left
                remax(
                    &mut ndp[cnt_left + 1],
                    cur_dp + multipliers[i] * a[cnt_left],
                );
                // take from right
                remax(
                    &mut ndp[cnt_left],
                    cur_dp + multipliers[i] * a[n - cnt_right - 1],
                );
            }
            dp = ndp;
        }
        let ans = *dp.iter().max().unwrap();
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
    #[case(vec![1,2,3], vec![3,2,1], 14)]
    #[case(vec![-5,-3,-3,-2,7,1], vec![-10,-5,3,4,6], 102)]
    fn case(#[case] nums: Vec<i32>, #[case] multipliers: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::maximum_score(nums, multipliers);
        assert_eq!(actual, expected);
    }
}
