//! Solution for https://leetcode.com/problems/contains-duplicate-iii
//! 220. Contains Duplicate III

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included};

impl Solution {
    pub fn contains_nearby_almost_duplicate(a: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let index_diff = index_diff as usize;
        let n = a.len();

        let mut set = BTreeSet::new();
        let mut l = 0;
        let mut r = 0;
        for i in 0..n {
            while r < n && r - i <= index_diff {
                set.insert((a[r], r));
                r += 1;
            }
            while i - l > index_diff {
                set.remove(&(a[l], l));
                l += 1;
            }

            set.remove(&(a[i], i));
            let min_val = a[i] - value_diff;
            let max_val = a[i] + value_diff + 1;
            let range = (
                Included((min_val, 0 as usize)),
                Excluded((max_val, 0 as usize)),
            );
            let it = set.range(range);
            if !it.last().is_none() {
                return true;
            }
            set.insert((a[i], i));
        }

        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,1], 3, 0, true)]
    #[case(vec![1,5,9,1,5,9], 2, 3, false)]
    fn case(
        #[case] nums: Vec<i32>,
        #[case] index_diff: i32,
        #[case] value_diff: i32,
        #[case] expected: bool,
    ) {
        let actual = Solution::contains_nearby_almost_duplicate(nums, index_diff, value_diff);
        assert_eq!(actual, expected);
    }
}
