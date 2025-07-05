//! Solution for https://leetcode.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows
//! 1439. Find the Kth Smallest Sum of a Matrix With Sorted Rows

use std::cmp::min;

fn rec(
    a: &Vec<Vec<i32>>,
    i: usize,
    mut cur_sum: i32,
    bound_sum: i32,
    bound_ans: usize,
    ans: &mut usize,
    min_after: &Vec<i32>,
) {
    if *ans > bound_ans {
        return;
    }
    if cur_sum + min_after[i] > bound_sum {
        return;
    }
    if i == a.len() {
        *ans += 1;
        return;
    }
    for j in 0..a[i].len() {
        cur_sum += a[i][j];
        if cur_sum <= bound_sum {
            rec(a, i + 1, cur_sum, bound_sum, bound_ans, ans, min_after);
        } else {
            return;
        }
        cur_sum -= a[i][j];
    }
}

impl Solution {
    pub fn kth_smallest(a: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = a.len();
        let m = a[0].len();
        let k = k as usize;

        let mut min_after = vec![0; n + 1];
        for i in (0..n).rev() {
            min_after[i] = min_after[i + 1] + a[i][0];
        }

        let cnt = |bound: i32| -> usize {
            let mut ans = 0;
            rec(&a, 0, 0, bound, k, &mut ans, &min_after);
            ans
        };

        let max_sum = a.iter().fold(0, |acc, e| acc + e.iter().sum::<i32>());
        let mut L = 0;
        let mut R = max_sum + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if cnt(M) < k {
                L = M;
            } else {
                R = M;
            }
        }
        R
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3,11],vec![2,4,6]], 5, 7)]
    // #[case(vec![vec![1,3,11],vec![2,4,6]], 9, 17)]
    // #[case(vec![vec![1,10,10],vec![1,4,5],vec![2,3,6]], 7, 9)]
    fn case(#[case] mat: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::kth_smallest(mat, k);
        assert_eq!(actual, expected);
    }
}
