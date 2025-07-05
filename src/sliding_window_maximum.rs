//! Solution for https://leetcode.com/problems/sliding-window-maximum
//! 239. Sliding Window Maximum

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(a: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = a.len();

        let push = |vec: &mut VecDeque<usize>, i: usize| {
            let x = a[i];
            while !vec.is_empty() && x >= a[*vec.back().unwrap()] {
                vec.pop_back();
            }
            vec.push_back(i);
        };

        let mut vec = VecDeque::new();
        for i in 0..k {
            push(&mut vec, i);
        }

        let mut ans = Vec::new();
        for i in 0..=n - k {
            ans.push(a[*vec.front().unwrap()]);

            if i == n - k {
                break;
            }

            push(&mut vec, i + k);
            if *vec.front().unwrap() == i {
                vec.pop_front();
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
    #[case(vec![1,3,-1,-3,5,3,6,7], 3, vec![3,3,5,5,6,7])]
    #[case(vec![1], 1, vec![1])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::max_sliding_window(nums, k);
        assert_eq!(actual, expected);
    }
}
