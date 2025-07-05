//! Solution for https://leetcode.com/problems/minimize-deviation-in-array
//! 1675. Minimize Deviation in Array

use std::collections::BTreeSet;

impl Solution {
    pub fn minimum_deviation(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut seq = vec![Vec::new(); n];
        for i in 0..n {
            let mut x = a[i];
            if x % 2 == 1 {
                seq[i].push(x);
                seq[i].push(x * 2);
            } else {
                while x % 2 == 0 {
                    assert!(x > 0);
                    seq[i].push(x);
                    x /= 2;
                }
                seq[i].push(x);
                seq[i].reverse();
            }
        }

        let mut set = BTreeSet::new();
        let mut ptr = vec![0; n];
        for i in 0..n {
            set.insert((seq[i][0], i));
            ptr[i] += 1;
        }
        let mut ans = i32::MAX;
        loop {
            let (min, i) = set.pop_first().unwrap();
            let max = set.last().unwrap().0;
            ans = ans.min(max - min);
            if ptr[i] == seq[i].len() {
                break;
            }
            set.insert((seq[i][ptr[i]], i));
            ptr[i] += 1;
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
    #[case(vec![1,2,3,4], 1)]
    #[case(vec![4,1,5,20,3], 3)]
    #[case(vec![2,10,8], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::minimum_deviation(nums);
        assert_eq!(actual, expected);
    }
}
