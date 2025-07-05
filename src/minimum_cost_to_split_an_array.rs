//! Solution for https://leetcode.com/problems/minimum-cost-to-split-an-array
//! 2547. Minimum Cost to Split an Array

impl Solution {
    pub fn min_cost(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();

        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        for i in 0..n {
            let mut cnt = vec![0; n];
            let mut cnt1 = 0;
            for j in i..n {
                let x = a[j];
                if cnt[x] == 0 {
                    cnt1 += 1;
                } else if cnt[x] == 1 {
                    cnt1 -= 1;
                }
                cnt[x] += 1;
                let cost = ((j - i + 1) as i32 - cnt1) + k;
                dp[j + 1] = dp[j + 1].min(dp[i] + cost);
            }
        }

        dp[n]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,1,2,1,3,3], 2, 8)]
    #[case(vec![1,2,1,2,1], 2, 6)]
    #[case(vec![1,2,1,2,1], 5, 10)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::min_cost(nums, k);
        assert_eq!(actual, expected);
    }
}
