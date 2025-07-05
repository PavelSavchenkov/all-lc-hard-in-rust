//! Solution for https://leetcode.com/problems/second-minimum-time-to-reach-destination
//! 2045. Second Minimum Time to Reach Destination

use std::collections::BTreeSet;

type WeightType = i32;

#[derive(Clone)]
struct EdgeT<T> {
    a: usize,
    b: usize,
    w: T,
}

type Edge = EdgeT<WeightType>;

struct Graph {
    es: Vec<Edge>,
    g: Vec<Vec<usize>>,
    n: usize,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            es: Vec::new(),
            g: vec![Vec::new(); n],
            n,
        }
    }

    fn add_edge(&mut self, e: &Edge) {
        let id = self.es.len();
        self.g[e.a].push(id);
        self.es.push(e.clone());
    }

    // custom
    fn dijkstra(&self, root: usize) -> Vec<(WeightType, WeightType)> {
        // mn1, mn2
        let mut dist = vec![(WeightType::MAX, WeightType::MAX); self.n];
        dist[root].0 = 0;

        let mut set = BTreeSet::new();
        set.insert((dist[root], root));
        while !set.is_empty() {
            let (_, v) = set.pop_first().unwrap();
            assert!(dist[v].0 < WeightType::MAX);
            for &id in &self.g[v] {
                let to = self.es[id].a ^ self.es[id].b ^ v;
                let w = self.es[id].w;
                for it in 0..2 {
                    let base_d = if it == 0 { dist[v].0 } else { dist[v].1 };
                    if base_d == WeightType::MAX {
                        continue;
                    }
                    let nd = base_d + w;
                    if nd > dist[to].1 || nd == dist[to].0 || nd == dist[to].1 {
                        continue;
                    }
                    set.remove(&(dist[to], to));
                    if nd < dist[to].0 {
                        dist[to].1 = dist[to].0;
                        dist[to].0 = nd;
                    } else if nd != dist[to].0 && nd < dist[to].1 {
                        dist[to].1 = nd;
                    }
                    set.insert((dist[to], to));
                }
            }
        }

        dist
    }
}

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let time = time as usize;
        let change = change as usize;

        let mut g = Graph::new(n);
        for e in &edges {
            let a = (e[0] - 1) as usize;
            let b = (e[1] - 1) as usize;
            g.add_edge(&Edge { a: a, b: b, w: 1 });
            g.add_edge(&Edge { a: b, b: a, w: 1 });
        }
        let dist = g.dijkstra(0);
        let (mn1, mn2) = dist[n - 1];
        assert!(mn1 < mn2);
        assert!(mn2 < WeightType::MAX);

        let d = mn2;
        let mut t = 0;
        for it in 0..d as usize {
            let t_rem = t % (change * 2);
            if t_rem >= change {
                t += change * 2 - t_rem;
            }
            t += time;
        }

        t as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![vec![1,2],vec![1,3],vec![1,4],vec![3,4],vec![4,5]], 3, 5, 13)]
    #[case(2, vec![vec![1,2]], 3, 2, 11)]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] time: i32,
        #[case] change: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::second_minimum(n, edges, time, change);
        assert_eq!(actual, expected);
    }
}
