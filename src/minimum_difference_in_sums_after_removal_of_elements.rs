//! Solution for https://leetcode.com/problems/minimum-difference-in-sums-after-removal-of-elements
//! 2163. Minimum Difference in Sums After Removal of Elements

use std::collections::BinaryHeap;

fn calc_min_pref_n(a: &Vec<i32>, n: usize) -> Vec<Option<i64>> {
    let mut heap = BinaryHeap::new();
    let mut dp = vec![None; a.len() + 1];
    let mut sum: i64 = 0;
    for i in 0..a.len() {
        heap.push(a[i]);
        sum += a[i] as i64;
        if heap.len() > n {
            sum -= heap.pop().unwrap() as i64;
        }
        if heap.len() == n {
            dp[i + 1] = Some(sum);
        }
    }
    dp
}

impl Solution {
    pub fn minimum_difference(mut a: Vec<i32>) -> i64 {
        assert!(a.len() % 3 == 0);
        let n = a.len() / 3;

        let pref = calc_min_pref_n(&a, n);
        a.reverse();
        a = a.iter().map(|&x| -x).collect();
        let suff = calc_min_pref_n(&a, n);

        let mut ans = i64::MAX;
        for len_pref in n..=2 * n {
            let sum_left = pref[len_pref].unwrap();
            let sum_right = -suff[3 * n - len_pref].unwrap();
            let cur = sum_left - sum_right;
            ans = ans.min(cur);
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
    #[case(vec![3,1,2], -1)]
    #[case(vec![7,9,5,8,1,3], 1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::minimum_difference(nums);
        assert_eq!(actual, expected);
    }
}
