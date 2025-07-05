//! Solution for https://leetcode.com/problems/find-the-maximum-length-of-a-good-subsequence-ii
//! 3177. Find the Maximum Length of a Good Subsequence II

impl Solution {
    pub fn maximum_length(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let k = k as usize;

        let mut vals = a.clone();
        vals.sort();
        vals.dedup();
        let a: Vec<_> = a.iter().map(|x| vals.binary_search(&x).unwrap()).collect();

        let run_eq = |dp: &mut Vec<usize>, ans: &mut usize| {
            let mut mx = vec![0; vals.len()];
            for i in 0..n {
                let x = a[i];
                dp[i] = dp[i].max(mx[x] + 1);
                mx[x] = mx[x].max(dp[i]);
                *ans = (*ans).max(dp[i]);
            }
        };

        let run_pref = |dp: &mut Vec<usize>, ans: &mut usize| {
            let mut ndp = dp.clone();
            let mut mx = 0;
            for i in 0..n {
                ndp[i] = ndp[i].max(mx + 1);
                mx = mx.max(dp[i]);
                *ans = (*ans).max(ndp[i]);
            }
            *dp = ndp;
        };

        let mut dp = vec![1; n];
        let mut ans = 1;

        run_eq(&mut dp, &mut ans);
        for it in 0..k {
            run_pref(&mut dp, &mut ans);
            run_eq(&mut dp, &mut ans);
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
    #[case(vec![1,2,1,1,3], 2, 4)]
    #[case(vec![1,2,3,4,5,1], 0, 2)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::maximum_length(nums, k);
        assert_eq!(actual, expected);
    }
}
