//! Solution for https://leetcode.com/problems/find-edges-in-shortest-paths
//! 3123. Find Edges in Shortest Paths

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

impl Solution {
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;

        let mut g = Graph::new(n);
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let w = e[2] as WeightType;
            g.add_edge(&Edge { a: a, b: b, w });
            g.add_edge(&Edge { a: b, b: a, w });
        }

        let dist_source = g.dijkstra(0);
        let dist_target = g.dijkstra(n - 1);

        let D = dist_source[n - 1];
        assert!(D == dist_target[0]);
        let mut ans = Vec::new();
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let w = e[2] as WeightType;
            let mut ok = false;
            if dist_source[a] < WeightType::MAX && dist_target[a] < WeightType::MAX {
                ok |= dist_source[a] + w + dist_target[b] == D;
                ok |= dist_source[b] + w + dist_target[a] == D;
            }
            ans.push(ok);
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
    #[case(6, vec![vec![0,1,4],vec![0,2,1],vec![1,3,2],vec![1,4,3],vec![1,5,1],vec![2,3,1],vec![3,5,3],vec![4,5,2]], vec![true,true,true,false,true,true,true,false])]
    #[case(4, vec![vec![2,0,1],vec![0,1,1],vec![0,3,4],vec![3,2,2]], vec![true,false,false,true])]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: Vec<bool>) {
        let actual = Solution::find_answer(n, edges);
        assert_eq!(actual, expected);
    }
}
