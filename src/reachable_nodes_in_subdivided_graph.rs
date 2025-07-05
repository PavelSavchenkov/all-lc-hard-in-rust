//! Solution for https://leetcode.com/problems/reachable-nodes-in-subdivided-graph
//! 882. Reachable Nodes In Subdivided Graph

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let n = n as usize;

        let mut g = Graph::new(n);
        let mut es = Vec::new();
        for e in &edges {
            let cnt = e[2] as WeightType;
            for it in 0..2 {
                let a = e[it] as usize;
                let b = e[it ^ 1] as usize;
                let e = Edge { a, b, w: cnt + 1 };
                g.add_edge(&e);
                if it == 0 {
                    es.push(e);
                }
            }
        }

        let max_moves = max_moves as WeightType;
        let dist = g.dijkstra(0);
        let mut ans = 0;
        for e in &es {
            let pref = 0.max(max_moves - dist[e.a]);
            let suff = 0.max(max_moves - dist[e.b]);
            let total = (e.w - 1).min(pref + suff);
            ans += total as i32;
        }
        for v in 0..n {
            if dist[v] <= max_moves {
                ans += 1;
            }
        }
        ans
    }
}

use std::collections::{BTreeSet, VecDeque};

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

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1,10],vec![0,2,1],vec![1,2,2]], 6, 3, 13)]
    #[case(vec![vec![0,1,4],vec![1,2,6],vec![0,2,8],vec![1,3,1]], 10, 4, 23)]
    #[case(vec![vec![1,2,4],vec![1,4,5],vec![1,3,1],vec![2,3,4],vec![3,4,5]], 17, 5, 1)]
    fn case(
        #[case] edges: Vec<Vec<i32>>,
        #[case] max_moves: i32,
        #[case] n: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::reachable_nodes(edges, max_moves, n);
        assert_eq!(actual, expected);
    }
}
