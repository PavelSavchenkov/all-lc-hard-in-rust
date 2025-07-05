//! Solution for https://leetcode.com/problems/minimize-or-of-remaining-elements-using-operations
//! 3022. Minimize OR of Remaining Elements Using Operations

const B: usize = 30;

fn bit(mask: i32, b: usize) -> bool {
    ((mask >> b) & 1) == 1
}

impl Solution {
    pub fn min_or_after_operations(a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = a.len();

        let mut prev_without = vec![vec![-1; B]; n];
        for i in 0..n {
            for b in 0..B {
                if !bit(a[i], b) {
                    prev_without[i][b] = i as i32;
                } else if i > 0 {
                    prev_without[i][b] = prev_without[i - 1][b];
                }
            }
        }

        let mut prev = vec![n as i32; n];
        let mut ans = 0;
        for b in (0..B).rev() {
            let mut dp = vec![i32::MAX; n + 1];
            dp[0] = 0;
            let mut pref_min = vec![i32::MAX; n + 1];
            pref_min[0] = 0;
            for i in 0..n {
                pref_min[i + 1] = pref_min[i];
                let p = prev[i].min(prev_without[i][b]);
                if p >= 0 {
                    let p = p as usize;
                    let pref = pref_min[p];
                    assert!(pref < i32::MAX);
                    dp[i + 1] = pref + i as i32;
                    let func = dp[i + 1] - (i + 1) as i32;
                    pref_min[i + 1] = pref_min[i + 1].min(func);
                }
            }
            if dp[n] as usize <= k {
                for i in 0..n {
                    prev[i] = prev[i].min(prev_without[i][b]);
                }
            } else {
                ans ^= 1 << b;
            }
        }

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
    #[case(vec![3,5,3,2,7], 2, 3)]
    #[case(vec![7,3,15,14,2,8], 4, 2)]
    #[case(vec![10,7,10,3,9,14,9,4], 1, 15)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::min_or_after_operations(nums, k);
        assert_eq!(actual, expected);
    }
}
