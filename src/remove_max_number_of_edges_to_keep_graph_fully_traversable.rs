//! Solution for https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable
//! 1579. Remove Max Number of Edges to Keep Graph Fully Traversable

#[derive(Clone)]
struct DSU {
    p: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self { p: vec![-1; n] }
    }

    fn get(&mut self, x: usize) -> usize {
        if self.p[x] < 0 {
            x
        } else {
            self.p[x] = self.get(self.p[x] as usize) as i32;
            self.p[x] as usize
        }
    }

    fn merge(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.get(x);
        y = self.get(y);
        if x == y {
            return false;
        }
        if -self.p[x] < -self.p[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.p[x] += self.p[y];
        self.p[y] = x as i32;
        true
    }

    fn comp_size(&mut self, x: usize) -> usize {
        let root = self.get(x);
        (-self.p[root]) as usize
    }
}

struct Edge {
    t: u32,
    a: usize,
    b: usize,
}

impl Edge {
    fn new(v: &Vec<i32>) -> Self {
        Self {
            t: v[0] as u32,
            a: (v[1] - 1) as usize,
            b: (v[2] - 1) as usize,
        }
    }
}

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut es = Vec::new();
        for e in &edges {
            es.push(Edge::new(e));
        }

        let mut dsu = DSU::new(n);
        let mut used = 0;
        for e in &es {
            if e.t == 3 {
                if dsu.merge(e.a, e.b) {
                    used += 1;
                }
            }
        }

        let mut dsu2 = vec![dsu.clone(), dsu.clone()];
        for e in &es {
            if e.t < 3 {
                if dsu2[e.t as usize - 1].merge(e.a, e.b) {
                    used += 1;
                }
            }
        }

        for it in 0..2 {
            if dsu2[it].comp_size(0) != n {
                return -1;
            }
        }

        (es.len() - used) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, vec![vec![3,1,2],vec![3,2,3],vec![1,1,3],vec![1,2,4],vec![1,1,2],vec![2,3,4]], 2)]
    #[case(4, vec![vec![3,1,2],vec![3,2,3],vec![1,1,4],vec![2,1,4]], 0)]
    #[case(4, vec![vec![3,2,3],vec![1,1,2],vec![2,3,4]], -1)]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::max_num_edges_to_remove(n, edges);
        assert_eq!(actual, expected);
    }
}
