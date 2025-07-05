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
