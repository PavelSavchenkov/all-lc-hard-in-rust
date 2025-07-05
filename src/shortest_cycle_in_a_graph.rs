//! Solution for https://leetcode.com/problems/shortest-cycle-in-a-graph
//! 2608. Shortest Cycle in a Graph

use std::collections::VecDeque;

impl Solution {
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = edges.len();

        let mut ans = usize::MAX;
        for to_rem in 0..m {
            let mut g = vec![Vec::new(); n];
            let mut source = n;
            let mut target = n;
            for i in 0..m {
                let a = edges[i][0] as usize;
                let b = edges[i][1] as usize;
                if i == to_rem {
                    source = a;
                    target = b;
                    continue;
                }
                g[a].push(b);
                g[b].push(a);
            }
            assert!(source < n);
            assert!(source != target);

            let mut dist = vec![usize::MAX; n];
            dist[source] = 0;
            let mut q = VecDeque::new();
            q.push_back(source);
            while !q.is_empty() {
                let v = q.pop_front().unwrap();
                for &to in &g[v] {
                    if dist[to] > dist[v] + 1 {
                        dist[to] = dist[v] + 1;
                        q.push_back(to);
                    }
                }
            }
            if dist[target] < usize::MAX {
                ans = ans.min(dist[target] + 1);
            }
        }

        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(7, vec![vec![0,1],vec![1,2],vec![2,0],vec![3,4],vec![4,5],vec![5,6],vec![6,3]], 3)]
    #[case(4, vec![vec![0,1],vec![0,2]], -1)]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::find_shortest_cycle(n, edges);
        assert_eq!(actual, expected);
    }
}
