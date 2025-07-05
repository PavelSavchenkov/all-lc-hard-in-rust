//! Solution for https://leetcode.com/problems/bus-routes
//! 815. Bus Routes

use std::collections::VecDeque;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let mut stops = Vec::new();
        for r in &routes {
            for &stop in r {
                stops.push(stop);
            }
        }
        stops.sort();
        stops.dedup();

        let routes: Vec<Vec<usize>> = routes
            .iter()
            .map(|r| {
                r.iter()
                    .map(|stop| stops.binary_search(&stop).unwrap())
                    .collect()
            })
            .collect();

        let N = stops.len() + routes.len();
        let mut g = vec![Vec::new(); N];
        for i in 0..routes.len() {
            for &stop in &routes[i] {
                g[stop].push((stops.len() + i, 1));
                g[stops.len() + i].push((stop, 0));
            }
        }

        let source = stops.binary_search(&source);
        if source.is_err() {
            return -1;
        }
        let source = source.unwrap();
        let target = stops.binary_search(&target);
        if target.is_err() {
            return -1;
        }
        let target = target.unwrap();

        // bfs 0/1
        let mut dist = vec![usize::MAX; N];
        let mut q = VecDeque::new();
        q.push_front(source);
        dist[source] = 0;
        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            for &(to, w) in &g[v] {
                if dist[to] <= dist[v] + w {
                    continue;
                }
                dist[to] = dist[v] + w;
                if w == 0 {
                    q.push_front(to);
                } else {
                    q.push_back(to);
                }
            }
        }

        if dist[target] == usize::MAX {
            return -1;
        }
        dist[target] as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2,7],vec![3,6,7]], 1, 6, 2)]
    #[case(vec![vec![7,12],vec![4,5,15],vec![6],vec![15,19],vec![9,12,13]], 15, 12, -1)]
    fn case(
        #[case] routes: Vec<Vec<i32>>,
        #[case] source: i32,
        #[case] target: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::num_buses_to_destination(routes, source, target);
        assert_eq!(actual, expected);
    }
}
