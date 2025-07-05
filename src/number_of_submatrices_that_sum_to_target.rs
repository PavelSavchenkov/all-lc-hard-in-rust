//! Solution for https://leetcode.com/problems/number-of-submatrices-that-sum-to-target
//! 1074. Number of Submatrices That Sum to Target

use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(a: Vec<Vec<i32>>, target: i32) -> i32 {
        let n = a.len();
        let m = a[0].len();
        let mut ans = 0;
        for i0 in 0..n {
            let mut row = vec![0; m];
            for i1 in i0..n {
                for j in 0..m {
                    row[j] += a[i1][j];
                }

                let mut cnt = HashMap::new();
                let mut pref = 0;
                for j in 0..m {
                    *cnt.entry(pref).or_insert(0) += 1;
                    pref += row[j];
                    ans += *cnt.entry(pref - target).or_default();
                }
            }
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
    #[case(vec![vec![0,1,0],vec![1,1,1],vec![0,1,0]], 0, 4)]
    #[case(vec![vec![1,-1],vec![-1,1]], 0, 5)]
    #[case(vec![vec![904]], 0, 0)]
    fn case(#[case] matrix: Vec<Vec<i32>>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::num_submatrix_sum_target(matrix, target);
        assert_eq!(actual, expected);
    }
}
