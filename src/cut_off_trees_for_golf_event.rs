//! Solution for https://leetcode.com/problems/cut-off-trees-for-golf-event
//! 675. Cut Off Trees for Golf Event

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let n = forest.len();
        let m = forest[0].len();

        if forest[0][0] == 0 {
            return -1;
        }

        let N = n * m;
        let mut g = Graph::new(N);
        for i in 0..n {
            for j in 0..m {
                if forest[i][j] == 0 {
                    continue;
                }
                let v = i * m + j;
                for k in 0..4 {
                    let ni = i as i32 + di[k];
                    let nj = j as i32 + dj[k];
                    if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if forest[ni][nj] == 0 {
                        continue;
                    }
                    let u = ni * m + nj;
                    g.add_edge(&Edge { a: v, b: u, w: 1 });
                }
            }
        }

        let mut ij = Vec::new();
        for i in 0..n {
            for j in 0..m {
                if forest[i][j] > 1 || (i == 0 && j == 0) {
                    ij.push((i, j));
                }
            }
        }
        ij.sort_by_key(|&(i, j)| forest[i][j]);

        let mut ans = 0;
        let mut v = 0;
        for (i, j) in &ij {
            let dist = g.ford_bellman(v);
            let u = i * m + j;
            if dist[u] == WeightType::MAX {
                return -1;
            }
            ans += dist[u];
            v = u;
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
    #[case(vec![vec![1,2,3],vec![0,0,4],vec![7,6,5]], 6)]
    #[case(vec![vec![1,2,3],vec![0,0,0],vec![7,6,5]], -1)]
    #[case(vec![vec![2,3,4],vec![0,0,5],vec![8,7,6]], 6)]
    fn case(#[case] forest: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::cut_off_tree(forest);
        assert_eq!(actual, expected);
    }
}
