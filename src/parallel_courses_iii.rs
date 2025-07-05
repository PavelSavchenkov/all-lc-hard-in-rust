//! Solution for https://leetcode.com/problems/parallel-courses-iii
//! 2050. Parallel Courses III

use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;

        let mut g = vec![Vec::new(); n];
        let mut in_deg = vec![0; n];
        for e in &relations {
            let a = (e[0] - 1) as usize;
            let b = (e[1] - 1) as usize;
            g[a].push(b);
            in_deg[b] += 1;
        }

        let mut heap = BinaryHeap::<(i32, usize)>::new();
        for v in 0..n {
            if in_deg[v] == 0 {
                heap.push((-time[v], v));
            }
        }
        let mut ans = 0;
        while !heap.is_empty() {
            let (mut t, v) = heap.pop().unwrap();
            t = -t;
            ans = ans.max(t);

            for &to in &g[v] {
                assert!(in_deg[to] > 0);
                in_deg[to] -= 1;
                if in_deg[to] == 0 {
                    heap.push((-(t + time[to]), to));
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
    #[case(3, vec![vec![1,3],vec![2,3]], vec![3,2,5], 8)]
    #[case(5, vec![vec![1,5],vec![2,5],vec![3,5],vec![3,4],vec![4,5]], vec![1,2,3,4,5], 12)]
    fn case(
        #[case] n: i32,
        #[case] relations: Vec<Vec<i32>>,
        #[case] time: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::minimum_time(n, relations, time);
        assert_eq!(actual, expected);
    }
}
