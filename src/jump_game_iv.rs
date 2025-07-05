//! Solution for https://leetcode.com/problems/jump-game-iv
//! 1345. Jump Game IV

use std::collections::VecDeque;

impl Solution {
    pub fn min_jumps(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut vals = a.clone();
        vals.sort();
        vals.dedup();
        let a: Vec<usize> = a.iter().map(|&x| vals.binary_search(&x).unwrap()).collect();

        let mut who_in_layer = vec![Vec::new(); vals.len()];
        for i in 0..n {
            who_in_layer[a[i]].push(i);
        }

        let mut is_layer_done = vec![false; vals.len()];
        let mut q = VecDeque::new();
        let mut dp = vec![usize::MAX; n];
        dp[0] = 0;
        q.push_back(0);
        while !q.is_empty() {
            let i = q.pop_front().unwrap();
            let mut next = Vec::<usize>::new();
            if !is_layer_done[a[i]] {
                is_layer_done[a[i]] = true;
                next.extend(who_in_layer[a[i]].iter());
            }
            if i > 0 {
                next.push(i - 1);
            }
            if i + 1 < n {
                next.push(i + 1);
            }
            for &j in &next {
                if dp[j] > dp[i] + 1 {
                    dp[j] = dp[i] + 1;
                    q.push_back(j);
                }
            }
        }
        dp[n - 1] as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![100,-23,-23,404,100,23,23,23,3,404], 3)]
    #[case(vec![7], 0)]
    #[case(vec![7,6,9,6,9,6,9,7], 1)]
    fn case(#[case] arr: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_jumps(arr);
        assert_eq!(actual, expected);
    }
}
