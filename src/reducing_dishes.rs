//! Solution for https://leetcode.com/problems/reducing-dishes
//! 1402. Reducing Dishes

impl Solution {
    pub fn max_satisfaction(mut a: Vec<i32>) -> i32 {
        let n = a.len();
        a.sort();

        let mut dp = vec![i32::MIN; n + 1];
        dp[0] = 0;
        for i in 0..n {
            let x = a[i];
            for have in (0..=i).rev() {
                let cur_dp = dp[have];
                assert!(cur_dp > i32::MIN);
                dp[have + 1] = dp[have + 1].max(dp[have] + x * (have + 1) as i32);
            }
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
    #[case(vec![-1,-8,0,5,-7], 14)]
    #[case(vec![4,3,2], 20)]
    #[case(vec![-1,-4,-5], 0)]
    fn case(#[case] satisfaction: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_satisfaction(satisfaction);
        assert_eq!(actual, expected);
    }
}
