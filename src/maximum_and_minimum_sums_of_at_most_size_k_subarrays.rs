//! Solution for https://leetcode.com/problems/maximum-and-minimum-sums-of-at-most-size-k-subarrays
//! 3430. Maximum and Minimum Sums of at Most Size K Subarrays

use std::collections::VecDeque;

fn calc_max(a: &Vec<i32>, k: usize) -> i64 {
    let n = a.len();
    let mut ans = 0;
    let mut sum_q = 0;
    let mut q = VecDeque::<(usize, i64)>::new();
    for i in 0..n {
        while !q.is_empty() && a[q.back().unwrap().0] <= a[i] {
            sum_q -= q.pop_back().unwrap().1;
        }
        let mut item = (i, 0);
        if !q.is_empty() {
            item.1 = (i - q.back().unwrap().0) as i64 * a[i] as i64;
        }
        sum_q += item.1;
        q.push_back(item);

        if q.front().unwrap().0 + k - 1 < i {
            sum_q -= q.pop_front().unwrap().1;
            if !q.is_empty() {
                sum_q -= q.front().unwrap().1;
                q.front_mut().unwrap().1 = 0;
            }
        }

        ans += sum_q;
        let l = if i + 1 >= k { i + 1 - k } else { 0 };
        let front = q.front().unwrap().0;
        ans += (front - l + 1) as i64 * a[front] as i64;
    }
    ans
}

impl Solution {
    pub fn min_max_subarray_sum(mut a: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;

        let mut ans = calc_max(&a, k);
        for x in &mut a {
            *x = -*x;
        }
        let neg = calc_max(&a, k);
        ans += -neg;
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
    #[case(vec![1,2,3], 2, 20)]
    #[case(vec![1,-3,1], 2, -6)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::min_max_subarray_sum(nums, k);
        assert_eq!(actual, expected);
    }
}
