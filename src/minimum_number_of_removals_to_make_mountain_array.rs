//! Solution for https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array
//! 1671. Minimum Number of Removals to Make Mountain Array

fn calc_pref_dp(a: &Vec<i32>) -> Vec<usize> {
    let n = a.len();
    let mut dp = vec![1; n];
    for i in 0..n {
        for j in 0..i {
            if a[j] < a[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    dp
}

impl Solution {
    pub fn minimum_mountain_removals(mut nums: Vec<i32>) -> i32 {
        let pref = calc_pref_dp(&nums);
        nums.reverse();
        let mut suff = calc_pref_dp(&nums);
        suff.reverse();
        nums.reverse();

        let n = nums.len();
        let mut ans = n - 1;
        for i in 0..n {
            let keep = pref[i] + suff[i] - 1;
            if pref[i] > 1 && suff[i] > 1 {
                ans = ans.min(n - keep);
            }
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
    #[case(vec![1,3,1], 0)]
    #[case(vec![2,1,1,5,6,2,3,1], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::minimum_mountain_removals(nums);
        assert_eq!(actual, expected);
    }
}
