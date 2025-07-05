//! Solution for https://leetcode.com/problems/max-value-of-equation
//! 1499. Max Value of Equation

use std::collections::VecDeque;

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = points.len();

        let mut ans = i32::MIN;
        let mut q = VecDeque::<(usize, i32)>::new();
        for i in 0..n {
            let x = points[i][0];
            let y = points[i][1];
            while !q.is_empty() && x - points[q.front().unwrap().0][0] > k {
                q.pop_front();
            }
            if !q.is_empty() {
                let (_, func) = *q.front().unwrap();
                let cur = x + y + func;
                ans = ans.max(cur);
            }
            let func_i = y - x;
            while !q.is_empty() && q.back().unwrap().1 <= func_i {
                q.pop_back();
            }
            q.push_back((i, func_i));
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
    #[case(vec![vec![1,3],vec![2,0],vec![5,10],vec![6,-10]], 1, 4)]
    #[case(vec![vec![0,0],vec![3,0],vec![9,2]], 3, 3)]
    fn case(#[case] points: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::find_max_value_of_equation(points, k);
        assert_eq!(actual, expected);
    }
}
