//! Solution for https://leetcode.com/problems/find-servers-that-handled-most-number-of-requests
//! 1606. Find Servers That Handled Most Number of Requests

use std::collections::BTreeSet;
use std::ops::Bound::{Included, Unbounded};

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k = k as usize;
        let n = arrival.len();
        assert!(load.len() == n);

        let mut free_servers = BTreeSet::new();
        for i in 0..k {
            free_servers.insert(i);
        }
        let mut when_complete = BTreeSet::<(u32, usize)>::new();
        let mut cnt_handled = vec![0; k];
        for j in 0..n {
            let t_arrive = arrival[j] as u32;
            while !when_complete.is_empty() {
                let &(t, i) = when_complete.first().unwrap();
                if t <= t_arrive {
                    free_servers.insert(i);
                    when_complete.pop_first();
                } else {
                    break;
                }
            }

            if free_servers.is_empty() {
                continue;
            }

            let i = j % k;
            let who_handle;
            let range = (Included(i), Unbounded);
            let mut it = free_servers.range(range);
            if let Some(&id) = it.next() {
                who_handle = id;
            } else {
                who_handle = *free_servers.first().unwrap();
            }
            assert!(who_handle < k);
            free_servers.remove(&who_handle);
            when_complete.insert((t_arrive + load[j] as u32, who_handle));
            cnt_handled[who_handle] += 1;
        }

        let max_handle = *cnt_handled.iter().max().unwrap();
        let max_servers: Vec<_> = (0..k).filter(|&i| cnt_handled[i] == max_handle).collect();
        max_servers.iter().map(|&i| i as i32).collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, vec![1,2,3,4,5], vec![5,2,3,3,3], vec![1])]
    #[case(3, vec![1,2,3,4], vec![1,2,1,2], vec![0])]
    #[case(3, vec![1,2,3], vec![10,12,11], vec![0,1,2])]
    fn case(
        #[case] k: i32,
        #[case] arrival: Vec<i32>,
        #[case] load: Vec<i32>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::busiest_servers(k, arrival, load);
        assert_eq!(actual, expected);
    }
}
