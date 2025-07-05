//! Solution for https://leetcode.com/problems/minimum-weighted-subgraph-with-the-required-paths
//! 2203. Minimum Weighted Subgraph With the Required Paths

use std::collections::{BTreeSet, VecDeque};

type WeightType = i64;

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

    fn ford_bellman(&self, root: usize) -> Vec<WeightType> {
        let mut dist = vec![WeightType::MAX; self.n];
        dist[root] = 0;

        let mut q = VecDeque::new();
        q.push_back(root);
        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            assert!(dist[v] < WeightType::MAX);
            for &id in &self.g[v] {
                let to = self.es[id].a ^ self.es[id].b ^ v;
                let w = self.es[id].w;
                let nd = dist[v] + w;
                if nd >= dist[to] {
                    continue;
                }
                dist[to] = nd;
                q.push_back(to);
            }
        }

        dist
    }

    fn dijkstra(&self, root: usize) -> Vec<WeightType> {
        let mut dist = vec![WeightType::MAX; self.n];
        dist[root] = 0;

        let mut set = BTreeSet::new();
        set.insert((dist[root], root));
        while !set.is_empty() {
            let (_, v) = set.pop_first().unwrap();
            assert!(dist[v] < WeightType::MAX);
            for &id in &self.g[v] {
                let to = self.es[id].a ^ self.es[id].b ^ v;
                let w = self.es[id].w;
                let nd = dist[v] + w;
                if nd >= dist[to] {
                    continue;
                }
                set.remove(&(dist[to], to));
                dist[to] = nd;
                set.insert((dist[to], to));
            }
        }

        dist
    }

    fn dijkstra_snd_min(&self, root: usize) -> Vec<(WeightType, WeightType)> {
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
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        let n = n as usize;
        let src1 = src1 as usize;
        let src2 = src2 as usize;
        let dest = dest as usize;

        let mut g = Graph::new(n);
        let mut g_rev = Graph::new(n);
        for e in &edges {
            let from = e[0] as usize;
            let to = e[1] as usize;
            let w = e[2] as WeightType;

            g.add_edge(&Edge { a: from, b: to, w });
            g_rev.add_edge(&Edge { a: to, b: from, w });
        }

        let dist1 = g.dijkstra(src1);
        let dist2 = g.dijkstra(src2);
        let dist_dest = g_rev.dijkstra(dest);

        let mut ans = i64::MAX;
        for v in 0..n {
            if dist1[v] >= WeightType::MAX {
                continue;
            }
            let mut cur = dist1[v];
            if dist2[v] >= WeightType::MAX {
                continue;
            }
            cur += dist2[v];
            if dist_dest[v] >= WeightType::MAX {
                continue;
            }
            cur += dist_dest[v];
            ans = ans.min(cur);
        }

        if ans == i64::MAX {
            ans = -1
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
    #[case(6, vec![vec![0,2,2],vec![0,5,6],vec![1,0,3],vec![1,4,5],vec![2,1,1],vec![2,3,3],vec![2,3,4],vec![3,4,2],vec![4,5,1]], 0, 1, 5, 9)]
    #[case(3, vec![vec![0,1,1],vec![2,1,1]], 0, 1, 2, -1)]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] src1: i32,
        #[case] src2: i32,
        #[case] dest: i32,
        #[case] expected: i64,
    ) {
        let actual = Solution::minimum_weight(n, edges, src1, src2, dest);
        assert_eq!(actual, expected);
    }
}
