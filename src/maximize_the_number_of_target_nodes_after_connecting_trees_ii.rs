//! Solution for https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii
//! 3373. Maximize the Number of Target Nodes After Connecting Trees II

use std::collections::VecDeque;

fn calc(es: &Vec<Vec<i32>>) -> (Vec<usize>, Vec<usize>) {
    let n = es.len() + 1;
    let mut g = vec![Vec::new(); n];
    for e in es {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push(b);
        g[b].push(a);
    }
    let mut depth = vec![0; n];
    let mut was = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    was[0] = true;
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        assert!(was[v]);
        for &to in &g[v] {
            if !was[to] {
                depth[to] = depth[v] ^ 1;
                was[to] = true;
                q.push_back(to);
            }
        }
    }

    let mut cnt = vec![0; 2];
    for v in 0..n {
        cnt[depth[v]] += 1;
    }
    (depth, cnt)
}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let (depth1, cnt1) = calc(&edges1);
        let (_, cnt2) = calc(&edges2);

        let mut ans = Vec::new();
        for v in 0..depth1.len() {
            let cur = cnt1[depth1[v]] + cnt2[0].max(cnt2[1]);
            ans.push(cur as i32);
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
    #[case(vec![vec![0,1],vec![0,2],vec![2,3],vec![2,4]], vec![vec![0,1],vec![0,2],vec![0,3],vec![2,7],vec![1,4],vec![4,5],vec![4,6]], vec![8,7,7,8,8])]
    #[case(vec![vec![0,1],vec![0,2],vec![0,3],vec![0,4]], vec![vec![0,1],vec![1,2],vec![2,3]], vec![3,6,6,6,6])]
    fn case(
        #[case] edges1: Vec<Vec<i32>>,
        #[case] edges2: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::max_target_nodes(edges1, edges2);
        assert_eq!(actual, expected);
    }
}
