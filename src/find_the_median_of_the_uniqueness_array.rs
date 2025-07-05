//! Solution for https://leetcode.com/problems/find-the-median-of-the-uniqueness-array
//! 3134. Find the Median of the Uniqueness Array

use std::collections::BTreeSet;

impl Solution {
    pub fn median_of_uniqueness_array(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut vals = a.clone();
        vals.sort();
        vals.dedup();
        let a: Vec<_> = a.iter().map(|x| vals.binary_search(&x).unwrap()).collect();

        let cnt_less_or_eq = |cnt: usize| -> i64 {
            if cnt == 0 {
                return 0;
            }
            let mut last = vec![n; vals.len()];
            let mut ans = 0;
            let mut set = BTreeSet::new();
            for i in (0..n).rev() {
                let x = a[i];
                if last[x] < n {
                    set.remove(&last[x]);
                }
                last[x] = i;
                set.insert(i);

                while set.len() > cnt + 1 {
                    set.pop_last();
                }
                assert!(!set.is_empty());

                let r = if set.len() == cnt + 1 {
                    *set.last().unwrap() - 1
                } else {
                    n - 1
                };
                if i <= r {
                    ans += (r - i + 1) as i64;
                }
            }
            ans
        };

        let N = n as i64 * (n + 1) as i64 / 2;
        let med = (N - 1) / 2;
        let mut L = 0;
        let mut R = n + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if cnt_less_or_eq(M) >= med + 1 {
                R = M;
            } else {
                L = M;
            }
        }
        R as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], 1)]
    #[case(vec![3,4,3,4,5], 2)]
    #[case(vec![4,3,5,4], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::median_of_uniqueness_array(nums);
        assert_eq!(actual, expected);
    }
}
