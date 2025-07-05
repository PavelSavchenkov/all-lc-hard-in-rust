//! Solution for https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree
//! 1489. Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree

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

#[derive(Clone, Copy, Debug)]
struct Edge {
    a: usize,
    b: usize,
    w: i32,
    i: usize,
}

fn dfs(
    v: usize,
    up_edge_id: usize,
    g: &Vec<Vec<Edge>>,
    fup: &mut Vec<i32>,
    depth: &mut Vec<i32>,
    critical: &mut Vec<i32>,
    pseudo_critical: &mut Vec<i32>,
) {
    assert!(depth[v] != -1);
    fup[v] = depth[v];
    for e in &g[v] {
        if e.i == up_edge_id {
            continue;
        }
        let to = e.a ^ e.b ^ v;
        if depth[to] == -1 {
            depth[to] = depth[v] + 1;
            dfs(to, e.i, g, fup, depth, critical, pseudo_critical);
            fup[v] = fup[v].min(fup[to]);
            if fup[to] > depth[v] {
                critical.push(e.i as i32);
            } else {
                pseudo_critical.push(e.i as i32);
            }
        } else {
            if depth[to] < depth[v] {
                pseudo_critical.push(e.i as i32);
                fup[v] = fup[v].min(depth[to]);
            }
        }
    }
}

impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut es = Vec::new();
        for i in 0..edges.len() {
            let a = edges[i][0] as usize;
            let b = edges[i][1] as usize;
            let w = edges[i][2];
            es.push(Edge { a, b, w, i });
        }

        es.sort_by_key(|e| e.w);

        let mut critical = Vec::new();
        let mut pseudo_critical = Vec::new();
        let mut dsu = DSU::new(n);
        let mut i = 0;
        while i < es.len() {
            let mut j = i;
            while j < es.len() && es[i].w == es[j].w {
                j += 1;
            }

            let mut nodes = Vec::new();
            let mut edges = Vec::new();
            for k in i..j {
                let a = dsu.get(es[k].a);
                let b = dsu.get(es[k].b);
                if a != b {
                    nodes.push(a);
                    nodes.push(b);
                    edges.push(Edge {
                        a,
                        b,
                        w: 0,
                        i: es[k].i,
                    });
                }
            }
            for k in i..j {
                dsu.merge(es[k].a, es[k].b);
            }

            nodes.sort();
            nodes.dedup();
            let mut g = vec![Vec::new(); nodes.len()];
            for e in &mut edges {
                e.a = nodes.binary_search(&e.a).unwrap();
                e.b = nodes.binary_search(&e.b).unwrap();
                g[e.a].push(*e);
                g[e.b].push(*e);
            }

            let mut fup = vec![i32::MAX; nodes.len()];
            let mut depth = vec![-1; nodes.len()];
            for v in 0..nodes.len() {
                if depth[v] == -1 {
                    depth[v] = 0;
                    dfs(
                        v,
                        es.len(),
                        &g,
                        &mut fup,
                        &mut depth,
                        &mut critical,
                        &mut pseudo_critical,
                    );
                }
            }

            i = j;
        }
        vec![critical, pseudo_critical]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![vec![0,1,1],vec![1,2,1],vec![2,3,2],vec![0,3,2],vec![0,4,3],vec![3,4,3],vec![1,4,6]], vec![vec![0,1],vec![2,3,4,5]])]
    #[case(4, vec![vec![0,1,1],vec![1,2,1],vec![2,3,1],vec![0,3,1]], vec![vec![],vec![0,1,2,3]])]
    #[case(4, vec![vec![0,1,1],vec![0,2,2],vec![2,3,3]], vec![vec![0,1,2]])]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::find_critical_and_pseudo_critical_edges(n, edges);
        assert_eq!(actual, expected);
    }
}
