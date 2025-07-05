//! Solution for https://leetcode.com/problems/maximize-consecutive-elements-in-an-array-after-modification
//! 3041. Maximize Consecutive Elements in an Array After Modification

impl Solution {
    pub fn max_selected_elements(mut a: Vec<i32>) -> i32 {
        a.sort();

        let mut na = Vec::new();
        for i in 0..a.len() {
            if i < 2 || a[i] != a[i - 2] {
                na.push(a[i]);
            }
        }
        a = na;

        let n = a.len();
        let mut dp = vec![vec![1; 2]; n];
        let mut ans = 1;
        for i in 0..n {
            for inc in 0..2 {
                let val = a[i] + inc as i32;
                for j in (0..i).rev() {
                    if i - j > 5 {
                        break;
                    }
                    for prev_inc in 0..2 {
                        let prev_val = a[j] + prev_inc as i32;
                        if prev_val + 1 == val {
                            let prev_dp = dp[j][prev_inc];
                            dp[i][inc] = dp[i][inc].max(prev_dp + 1);
                        }
                    }
                }
                ans = ans.max(dp[i][inc]);
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
    #[case(vec![2,1,5,1,1], 3)]
    #[case(vec![1,4,7,10], 1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_selected_elements(nums);
        assert_eq!(actual, expected);
    }
}
