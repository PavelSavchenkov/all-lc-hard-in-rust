//! Solution for https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists
//! 632. Smallest Range Covering Elements from K Lists

use std::collections::BTreeSet;

impl Solution {
    pub fn smallest_range(a: Vec<Vec<i32>>) -> Vec<i32> {
        let k = a.len();

        let mut L = -i32::pow(10, 6);
        let mut R = i32::pow(10, 6);
        let mut set = BTreeSet::<(i32, usize)>::new();
        let mut a_ptr = vec![0 as usize; k];
        for i in 0..k {
            set.insert((a[i][0], i));
            a_ptr[i] += 1;
        }
        loop {
            assert!(set.len() == k);
            let (min, min_i) = *set.first().unwrap();
            let max = set.last().unwrap().0;

            if max - min < R - L || (max - min == R - L && min < L) {
                L = min;
                R = max;
            }

            if a_ptr[min_i] == a[min_i].len() {
                break;
            }
            set.pop_first();
            set.insert((a[min_i][a_ptr[min_i]], min_i));
            a_ptr[min_i] += 1;
        }

        vec![L, R]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![4,10,15,24,26],vec![0,9,12,20],vec![5,18,22,30]], vec![20,24])]
    #[case(vec![vec![1,2,3],vec![1,2,3],vec![1,2,3]], vec![1,1])]
    fn case(#[case] nums: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::smallest_range(nums);
        assert_eq!(actual, expected);
    }
}
