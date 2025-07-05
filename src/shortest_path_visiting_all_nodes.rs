//! Solution for https://leetcode.com/problems/shortest-path-visiting-all-nodes
//! 847. Shortest Path Visiting All Nodes

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();

        let mut dp = vec![vec![usize::MAX; n]; 1 << n];
        let mut q = VecDeque::new();
        for i in 0..n {
            let mask = 1 << i;
            dp[mask][i] = 0;
            q.push_back((mask, i));
        }
        while !q.is_empty() {
            let (mask, v) = q.pop_front().unwrap();
            let cur_dp = dp[mask][v];
            assert!(cur_dp < usize::MAX);
            for &to in &graph[v] {
                let to = to as usize;
                let nmask = mask | (1 << to);
                if dp[nmask][to] > cur_dp + 1 {
                    dp[nmask][to] = cur_dp + 1;
                    q.push_back((nmask, to));
                }
            }
        }

        *dp[(1 << n) - 1].iter().min().unwrap() as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2,3],vec![0],vec![0],vec![0]], 4)]
    #[case(vec![vec![1],vec![0,2,4],vec![1,3,4],vec![2],vec![1,2]], 4)]
    fn case(#[case] graph: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::shortest_path_length(graph);
        assert_eq!(actual, expected);
    }
}
