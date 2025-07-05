//! Solution for https://leetcode.com/problems/find-maximum-non-decreasing-array-length
//! 2945. Find Maximum Non-decreasing Array Length

use std::collections::BTreeSet;

impl Solution {
    pub fn find_maximum_length(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + a[i] as i64;
        }

        let mut max_cnt = vec![1; n + 1];
        let mut min_last = vec![i64::MAX; n + 1];
        max_cnt[0] = 0;
        min_last[0] = 0;
        let mut by_func = BTreeSet::new();
        let mut js = BTreeSet::new();
        by_func.insert((min_last[0] + pref[0], 0));
        for i in 0..n {
            while !by_func.is_empty() {
                let first = by_func.first().unwrap();
                if first.0 <= pref[i + 1] {
                    js.insert(first.1);
                    by_func.pop_first();
                } else {
                    break;
                }
            }
            assert!(!js.is_empty());
            let j = *js.last().unwrap();
            min_last[i + 1] = pref[i + 1] - pref[j];
            max_cnt[i + 1] = max_cnt[j] + 1;
            by_func.insert((min_last[i + 1] + pref[i + 1], i + 1));
        }
        max_cnt[n]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![5,2,2], 1)]
    #[case(vec![1,2,3,4], 4)]
    #[case(vec![4,3,2,6], 3)]
    #[case(vec![147,633,535,183,886], 4)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::find_maximum_length(nums);
        assert_eq!(actual, expected);
    }
}
