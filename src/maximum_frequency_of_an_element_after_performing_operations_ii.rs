//! Solution for https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-ii
//! 3347. Maximum Frequency of an Element After Performing Operations II

impl Solution {
    pub fn max_frequency(mut a: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        a.sort();
        let n = a.len();
        let ops = num_operations as usize;

        let cnt_seg = |l: i32, r: i32| -> usize {
            let l = a.partition_point(|&x| x < l);
            let r = a.partition_point(|&x| x <= r);
            r - l
        };

        let mut ans = 1;
        for i in 0..n {
            let cnt_eq = cnt_seg(a[i], a[i]);
            let cnt = cnt_seg(a[i] - k, a[i] + k) - cnt_eq;
            ans = ans.max(cnt.min(ops) + cnt_eq);
        }

        for i in 0..n {
            let cnt = cnt_seg(a[i], a[i] + 2 * k);
            ans = ans.max(cnt.min(ops));
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
    #[case(vec![1,4,5], 1, 2, 2)]
    #[case(vec![5,11,20,20], 5, 1, 2)]
    fn case(
        #[case] nums: Vec<i32>,
        #[case] k: i32,
        #[case] num_operations: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::max_frequency(nums, k, num_operations);
        assert_eq!(actual, expected);
    }
}
