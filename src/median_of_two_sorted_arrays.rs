//! Solution for https://leetcode.com/problems/median-of-two-sorted-arrays
//! 4. Median of Two Sorted Arrays

use std::i32;

fn try_get_kth_from_a(a: &Vec<i32>, b: &Vec<i32>, k: i32) -> i32 {
    let mut L = -1;
    let mut R = a.len() as i32;
    while L != R - 1 {
        let M = (L + R) / 2;

        let val = a[M as usize];
        let have = M + 1;
        let need = k + 1 - have;
        if need <= 0 || (need <= b.len() as i32 && b[need as usize - 1] <= val) {
            R = M;
        } else {
            L = M;
        }
    }
    if R < a.len() as i32 {
        a[R as usize]
    } else {
        i32::MAX
    }
}

// min value such as there are at least k+1 other values <= than ours
fn get_kth(a: &Vec<i32>, b: &Vec<i32>, k: i32) -> i32 {
    let ans = try_get_kth_from_a(a, b, k).min(try_get_kth_from_a(b, a, k));
    assert!(ans < i32::MAX);
    return ans;
}

impl Solution {
    pub fn find_median_sorted_arrays(a: Vec<i32>, b: Vec<i32>) -> f64 {
        let n = a.len();
        let m = b.len();
        let len = (n + m) as i32;
        if len % 2 == 1 {
            return get_kth(&a, &b, len / 2) as f64;
        } else {
            let left = get_kth(&a, &b, len / 2 - 1) as f64;
            let right = get_kth(&a, &b, len / 2) as f64;
            return (left + right) / 2.0;
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3], vec![2], 2.00000)]
    #[case(vec![1,2], vec![3,4], 2.50000)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: f64) {
        let actual = Solution::find_median_sorted_arrays(nums1, nums2);
        assert!(
            (actual - expected).abs() < 1e-5,
            "Assertion failed: actual {actual:.5} but expected {expected:.5}. Diff is more than 1e-5."
        );
    }
}
