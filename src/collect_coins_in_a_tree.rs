//! Solution for https://leetcode.com/problems/collect-coins-in-a-tree
//! 2603. Collect Coins in a Tree

use std::collections::BTreeSet;

impl Solution {
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let coins: Vec<bool> = coins.iter().map(|&c| c == 1).collect();
        let n = coins.len();

        let mut deg = vec![0; n];
        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
            deg[a] += 1;
            deg[b] += 1;
        }

        let mut by_deg = BTreeSet::new();
        for v in 0..n {
            if deg[v] == 1 {
                by_deg.insert((deg[v], v));
            }
        }
        let mut killed = vec![false; n];
        while !by_deg.is_empty() {
            let (_, v) = by_deg.pop_first().unwrap();
            assert!(deg[v] <= 1);
            if coins[v] {
                continue;
            }
            assert!(!killed[v]);
            killed[v] = true;
            for &to in &g[v] {
                if !killed[to] {
                    if deg[to] == 1 {
                        by_deg.remove(&(deg[to], to));
                    }
                    deg[to] -= 1;
                    if deg[to] == 1 {
                        by_deg.insert((deg[to], to));
                    }
                }
            }
        }

        // now all leaves have a coin
        // --> remove them as well
        let mut leaves_with_coin = Vec::new();
        for v in 0..n {
            if deg[v] == 1 && coins[v] {
                leaves_with_coin.push(v);
            }
        }
        for &v in &leaves_with_coin {
            assert!(!killed[v]);
            for &to in &g[v] {
                if !killed[to] {
                    deg[to] -= 1;
                }
            }
            killed[v] = true;
        }

        // now we have to be at dist 1 from all remaining leaves at some point
        // --> it's enough to visit all non-leaf nodes
        for v in 0..n {
            if deg[v] == 1 {
                killed[v] = true;
            }
        }

        let mut ans = 0;
        for v in 0..n {
            if killed[v] {
                continue;
            }
            for &to in &g[v] {
                if !killed[to] {
                    ans += 1;
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
    #[case(vec![1,0,0,0,0,1], vec![vec![0,1],vec![1,2],vec![2,3],vec![3,4],vec![4,5]], 2)]
    #[case(vec![0,0,0,1,1,0,0,1], vec![vec![0,1],vec![0,2],vec![1,3],vec![1,4],vec![2,5],vec![5,6],vec![5,7]], 2)]
    #[case(vec![1,1,0,0,1,0,0,1,0,1,0,0,1,0,0,1,0,0,1], vec![vec![0,1],vec![1,2],vec![2,3],vec![3,4],vec![4,5],vec![5,6],vec![6,7],vec![7,8],vec![8,9],vec![8,10],vec![9,11],vec![10,12],vec![11,13],vec![13,14],vec![13,15],vec![15,16],vec![16,17],vec![17,18]], 22)]
    fn case(#[case] coins: Vec<i32>, #[case] edges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::collect_the_coins(coins, edges);
        assert_eq!(actual, expected);
    }
}
