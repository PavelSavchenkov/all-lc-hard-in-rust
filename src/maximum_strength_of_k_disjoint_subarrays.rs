//! Solution for https://leetcode.com/problems/maximum-strength-of-k-disjoint-subarrays
//! 3077. Maximum Strength of K Disjoint Subarrays

impl Solution {
    pub fn maximum_strength(a: Vec<i32>, k: i32) -> i64 {
        let n = a.len();
        let k = k as usize;

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + a[i] as i64;
        }

        let mut dp = vec![0; n + 1];
        for c in (1..=k).rev() {
            let mut coef = c as i64;
            if c % 2 == 0 {
                coef = -coef;
            }
            let mut ndp = vec![i64::MIN; n + 1];
            let mut mx = i64::MIN;
            for i in 0..=n {
                if mx > i64::MIN {
                    ndp[i] = coef * pref[i] + mx;
                }
                if i > 0 {
                    let prev = ndp[i - 1];
                    ndp[i] = ndp[i].max(prev);
                }
                if dp[i] > i64::MIN {
                    mx = mx.max(dp[i] - coef * pref[i]);
                }
            }
            dp = ndp;
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
    #[case(vec![1,2,3,-1,2], 3, 22)]
    #[case(vec![12,-2,-2,-2,-2], 5, 64)]
    #[case(vec![-1,-2,-3], 1, -1)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::maximum_strength(nums, k);
        assert_eq!(actual, expected);
    }
}
