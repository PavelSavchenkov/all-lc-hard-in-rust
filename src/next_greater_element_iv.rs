//! Solution for https://leetcode.com/problems/next-greater-element-iv
//! 2454. Next Greater Element IV

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

impl Solution {
    pub fn second_greater_element(a: Vec<i32>) -> Vec<i32> {
        let n = a.len();

        let mut ord: Vec<_> = (0..n).collect();
        ord.sort_by_key(|&i| -a[i]);

        let mut ans = vec![-1; n];
        let mut set = BTreeSet::<usize>::new();
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && a[ord[i]] == a[ord[j]] {
                j += 1;
            }

            for k in i..j {
                let pos = ord[k];
                let range = (Excluded(pos), Unbounded);
                let mut it = set.range(range);
                it.next();
                let pos2 = it.next();
                if pos2.is_some() {
                    ans[ord[k]] = a[*pos2.unwrap()];
                }
            }

            for k in i..j {
                let pos = ord[k];
                set.insert(pos);
            }

            i = j;
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
    #[case(vec![2,4,0,9,6], vec![9,6,6,-1,-1])]
    #[case(vec![3,3], vec![-1,-1])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::second_greater_element(nums);
        assert_eq!(actual, expected);
    }
}
