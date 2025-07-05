//! Solution for https://leetcode.com/problems/word-ladder-ii
//! 126. Word Ladder II

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
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
                if nd <= dist[to] {
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

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn go(
    dist: &Vec<WeightType>,
    g: &Graph,
    v: usize,
    path: &mut Vec<usize>,
    paths: &mut Vec<Vec<usize>>,
) {
    if dist[v] == 0 {
        paths.push(path.clone());
        return;
    }
    for &id in &g.g[v] {
        let to = g.es[id].a ^ g.es[id].b ^ v;
        if dist[to] + 1 == dist[v] {
            path.push(to);
            go(dist, g, to, path, paths);
            path.pop();
        }
    }
}

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        mut word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        word_list.push(begin_word.clone());
        let begin_word = to_u8(&begin_word);
        let end_word = to_u8(&end_word);
        let word_list: Vec<_> = word_list.iter().map(|w| to_u8(w)).collect();

        let mut pos_of = HashMap::<u64, usize>::new();
        for i in 0..word_list.len() {
            pos_of.insert(get_hash(&word_list[i]), i);
        }

        let mut g = Graph::new(word_list.len());
        for i in 0..word_list.len() {
            let w = &word_list[i];
            for j in 0..w.len() {
                for c in 0..26 {
                    let ch = b'a' + c;
                    if w[j] == ch {
                        continue;
                    }
                    let mut nw = w.clone();
                    nw[j] = ch;
                    let pos = pos_of.get(&get_hash(&nw));
                    if !pos.is_none() {
                        let pos = *pos.unwrap();
                        g.add_edge(&Edge { a: i, b: pos, w: 1 });
                    }
                }
            }
        }
        let end = pos_of.get(&get_hash(&end_word));
        if end.is_none() {
            return Vec::new();
        }
        let end = *end.unwrap();

        let begin = *pos_of.get(&get_hash(&begin_word)).unwrap();
        let dist = g.ford_bellman(begin);
        if dist[end] == WeightType::MAX {
            return Vec::new();
        }

        let mut paths = Vec::<Vec<usize>>::new();
        let mut path = Vec::<usize>::new();
        path.push(end);
        go(&dist, &g, end, &mut path, &mut paths);

        let paths = paths
            .iter()
            .map(|p| p.iter().rev().map(|&w| from_u8(&word_list[w])).collect())
            .collect();

        paths
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("hit", "cog", vec!["hot".into(),"dot".into(),"dog".into(),"lot".into(),"log".into(),"cog".into()], vec![vec!["hit".into(),"hot".into(),"dot".into(),"dog".into(),"cog".into()],vec!["hit".into(),"hot".into(),"lot".into(),"log".into(),"cog".into()]])]
    #[case("hit", "cog", vec!["hot".into(),"dot".into(),"dog".into(),"lot".into(),"log".into()], vec![])]
    fn case(
        #[case] begin_word: String,
        #[case] end_word: String,
        #[case] word_list: Vec<String>,
        #[case] expected: Vec<Vec<String>>,
    ) {
        let actual = Solution::find_ladders(begin_word, end_word, word_list);
        assert_eq!(actual, expected);
    }
}
