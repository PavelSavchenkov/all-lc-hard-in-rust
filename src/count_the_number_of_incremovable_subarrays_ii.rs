//! Solution for https://leetcode.com/problems/count-the-number-of-incremovable-subarrays-ii
//! 2972. Count the Number of Incremovable Subarrays II

impl Solution {
    pub fn incremovable_subarray_count(mut a: Vec<i32>) -> i64 {
        a.insert(0, i32::MIN);
        a.push(i32::MAX);
        let n = a.len();

        let mut prefs = Vec::new();
        for i in 0..n {
            if i > 0 && a[i] <= a[i - 1] {
                break;
            }
            prefs.push(a[i]);
        }

        let mut ptr = prefs.len();
        let mut ans = 0;
        for i in (0..n).rev() {
            if i + 1 < n && a[i] >= a[i + 1] {
                break;
            }
            while ptr > 0 && prefs[ptr - 1] >= a[i] {
                ptr -= 1;
            }
            ans += ptr as i64;
        }
        if prefs.len() == n {
            ans -= (n - 1) as i64;
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
    #[case(vec![1,2,3,4], 10)]
    #[case(vec![6,5,7,8], 7)]
    #[case(vec![8,7,6,6], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::incremovable_subarray_count(nums);
        assert_eq!(actual, expected);
    }
}
