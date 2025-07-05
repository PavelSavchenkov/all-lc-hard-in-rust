//! Solution for https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k
//! 862. Shortest Subarray with Sum at Least K

use std::collections::BTreeSet;

impl Solution {
    pub fn shortest_subarray(a: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let n = a.len();

        let mut pref = vec![0 as i64; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + a[i] as i64;
        }

        // [l; r), r - l <= len
        let exists = |len: usize| -> bool {
            let mut set = BTreeSet::<(i64, usize)>::new();
            for r in 0..=n {
                if r >= len + 1 {
                    assert!(set.len() == len + 1);
                    set.remove(&(pref[r - len - 1], r - len - 1));
                }
                if !set.is_empty() && set.first().unwrap().0 <= pref[r] - k {
                    return true;
                }
                set.insert((pref[r], r));
            }
            false
        };

        let mut L = 0;
        let mut R = n + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if exists(M) {
                R = M;
            } else {
                L = M;
            }
        }
        
        if R == n + 1 {
            return -1
        }
        R as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1], 1, 1)]
    #[case(vec![1,2], 4, -1)]
    #[case(vec![2,-1,2], 3, 3)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::shortest_subarray(nums, k);
        assert_eq!(actual, expected);
    }
}
