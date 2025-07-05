//! Solution for https://leetcode.com/problems/make-array-strictly-increasing
//! 1187. Make Array Strictly Increasing

impl Solution {
    pub fn make_array_increasing(a: Vec<i32>, mut b: Vec<i32>) -> i32 {
        let n = a.len();
        b.sort();
        b.dedup();
        let m = b.len();

        let mut dp = vec![1; m + 1];
        dp[m] = 0;
        for i in 1..n {
            let mut ndp = vec![usize::MAX; m + 1];
            let mut pref_min = usize::MAX;
            for j in 0..m {
                let mut cur = pref_min;
                if a[i - 1] < b[j] {
                    cur = cur.min(dp[m]);
                }
                pref_min = pref_min.min(dp[j]);
                if cur == usize::MAX {
                    continue;
                }
                ndp[j] = cur + 1;
            }
            for j in 0..m {
                if b[j] < a[i] {
                    ndp[m] = ndp[m].min(dp[j]);
                }
            }
            if a[i - 1] < a[i] {
                ndp[m] = ndp[m].min(dp[m]);
            }
            dp = ndp;
        }
        let ans = *dp.iter().min().unwrap();
        if ans == usize::MAX {
            return -1;
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
    #[case(vec![1,5,3,6,7], vec![1,3,2,4], 1)]
    #[case(vec![1,5,3,6,7], vec![4,3,1], 2)]
    #[case(vec![1,5,3,6,7], vec![1,6,3,3], -1)]
    fn case(#[case] arr1: Vec<i32>, #[case] arr2: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::make_array_increasing(arr1, arr2);
        assert_eq!(actual, expected);
    }
}
