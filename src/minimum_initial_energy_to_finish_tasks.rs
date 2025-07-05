//! Solution for https://leetcode.com/problems/minimum-initial-energy-to-finish-tasks
//! 1665. Minimum Initial Energy to Finish Tasks

use std::collections::BTreeSet;

impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        let n = tasks.len();

        let mut actual = Vec::with_capacity(n);
        let mut minimum = Vec::with_capacity(n);
        for i in 0..n {
            actual.push(tasks[i][0]);
            minimum.push(tasks[i][1]);
        }

        let can_all = |mut E: i32| -> bool {
            let mut set = BTreeSet::new();
            for i in 0..n {
                set.insert((minimum[i] - actual[i], i));
            }
            while !set.is_empty() {
                let (min, i) = set.pop_first().unwrap();
                if min > E {
                    return false;
                }
                E += actual[i];
            }
            true
        };

        let mut L = -1;
        let mut R = *minimum.iter().max().unwrap() + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if can_all(M) {
                R = M;
            } else {
                L = M;
            }
        }
        let spent: i32 = actual.iter().sum();
        R + spent
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2],vec![2,4],vec![4,8]], 8)]
    #[case(vec![vec![1,3],vec![2,4],vec![10,11],vec![10,12],vec![8,9]], 32)]
    #[case(vec![vec![1,7],vec![2,8],vec![3,9],vec![4,10],vec![5,11],vec![6,12]], 27)]
    fn case(#[case] tasks: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::minimum_effort(tasks);
        assert_eq!(actual, expected);
    }
}
