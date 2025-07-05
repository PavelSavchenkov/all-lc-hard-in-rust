//! Solution for https://leetcode.com/problems/maximum-number-of-ways-to-partition-an-array
//! 2025. Maximum Number of Ways to Partition an Array

use std::collections::HashMap;

impl Solution {
    pub fn ways_to_partition(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let mut pref = vec![0 as i64; n];
        for i in 0..n {
            pref[i] += a[i] as i64;
            if i > 0 {
                pref[i] += pref[i - 1];
            }
        }

        let S = pref[n - 1];

        let mut left = HashMap::<i64, usize>::new();
        let mut right = HashMap::<i64, usize>::new();
        for i in 0..n - 1 {
            *right.entry(pref[i]).or_default() += 1;
        }

        let mut ans: usize = 0;

        // don't change
        if S % 2 == 0 {
            let half = S / 2;
            ans += right.get(&half).unwrap_or(&0);
        }

        // change a[i]
        for i in 0..n {
            let new_S = S - a[i] as i64 + k as i64;
            if new_S % 2 == 0 {
                let half = new_S / 2;
                let mut cur_ans = 0;
                cur_ans += *left.get(&half).unwrap_or(&0);

                let change = -a[i] + k;
                cur_ans += *right.get(&(half - change as i64)).unwrap_or(&0);

                ans = ans.max(cur_ans);
            }

            if i < n - 1 {
                *right.get_mut(&pref[i]).unwrap() -= 1;
                *left.entry(pref[i]).or_default() += 1;
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
    #[case(vec![2,-1,2], 3, 1)]
    #[case(vec![0,0,0], 1, 2)]
    #[case(vec![22,4,-25,-20,-15,15,-16,7,19,-10,0,-13,-14], -33, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::ways_to_partition(nums, k);
        assert_eq!(actual, expected);
    }
}
