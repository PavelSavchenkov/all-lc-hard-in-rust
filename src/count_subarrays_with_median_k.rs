//! Solution for https://leetcode.com/problems/count-subarrays-with-median-k
//! 2488. Count Subarrays With Median K

use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(mut a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        for x in &mut a {
            *x = (*x - k).signum();
        }

        let pos_k = a.iter().position(|&x| x == 0).unwrap();

        let mut ans = 0;
        for it in 0..2 {
            let mut cnt_pref = HashMap::<i32, usize>::new();
            let mut pref: i32 = 0;
            for i in 0..n {
                if i <= pos_k {
                    *cnt_pref.entry(pref).or_insert(0) += 1;
                }
                pref += a[i];
                if i >= pos_k {
                    ans += *cnt_pref.entry(pref).or_default();
                }
            }

            for x in &mut a {
                if *x == 0 {
                    *x = -1;
                }
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
    #[case(vec![3,2,1,4,5], 4, 3)]
    #[case(vec![2,3,1], 3, 1)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::count_subarrays(nums, k);
        assert_eq!(actual, expected);
    }
}
