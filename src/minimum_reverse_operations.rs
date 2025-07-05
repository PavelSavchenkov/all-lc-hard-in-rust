//! Solution for https://leetcode.com/problems/minimum-reverse-operations
//! 2612. Minimum Reverse Operations

use std::collections::{BTreeSet, VecDeque};
use std::ops::Bound::{Included};

impl Solution {
    pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
        let p = p as usize;
        let n = n as usize;

        let mut is_banned = vec![false; n];
        for &i in &banned {
            is_banned[i as usize] = true;
        }

        let mut dist = vec![-1; n];
        dist[p] = 0;
        let mut q = VecDeque::new();
        q.push_front(p as i32);
        let mut unused = vec![BTreeSet::new(); 2];
        for i in 0..n {
            if !is_banned[i] && i != p {
                unused[i % 2].insert(i as i32);
            }
        }
        while !q.is_empty() {
            let i = q.pop_front().unwrap();

            let L = (i - k + 1).max(0);
            let R = i.min(n as i32 - k);
            let L = 2 * L + k - 1 - i;
            let R = 2 * R + k - 1 - i;
            assert!(L >= 0);
            assert!(R < n as i32);
            let parity = (L % 2) as usize;
            // eprintln!("i = {}, L = {}, R = {}", i, L, R);

            let range = (Included(L), Included(R));
            let mut it = unused[parity].range(range);
            let mut all_i_next = Vec::new();
            loop {
                let item = it.next();
                if item.is_none() {
                    break
                }
                let i_next = *item.unwrap();
                assert!(dist[i_next as usize] == -1);
                dist[i_next as usize] = dist[i as usize] + 1;
                all_i_next.push(i_next);
            }
            for &i_next in &all_i_next {
                unused[parity].remove(&i_next);
                q.push_back(i_next);
            }
        }

        dist
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, 0, vec![1,2], 4, vec![0,-1,-1,1])]
    #[case(5, 0, vec![2,4], 3, vec![0,-1,-1,-1,-1])]
    #[case(4, 2, vec![0,1,3], 1, vec![-1,-1,0,-1])]
    #[case(9, 7, vec![8, 0, 5], 4, vec![-1,2,3,2,1,-1,1,0,-1])]
    fn case(
        #[case] n: i32,
        #[case] p: i32,
        #[case] banned: Vec<i32>,
        #[case] k: i32,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::min_reverse_operations(n, p, banned, k);
        assert_eq!(actual, expected);
    }
}
