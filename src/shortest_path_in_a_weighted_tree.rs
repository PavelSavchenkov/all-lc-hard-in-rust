//! Solution for https://leetcode.com/problems/shortest-path-in-a-weighted-tree
//! 3515. Shortest Path in a Weighted Tree

struct Tree {
    n: usize,
    g: Vec<Vec<(usize, i32)>>,
    tin: Vec<usize>,
    tout: Vec<usize>,
    par: Vec<usize>,
    par_w: Vec<i32>,
    root_w: Vec<i32>,
}

impl Tree {
    fn new(es: &Vec<Vec<i32>>) -> Self {
        let n = es.len() + 1;
        let mut this = Self {
            n,
            g: vec![Vec::new(); n],
            tin: vec![0; n],
            tout: vec![0; n],
            par: vec![0; n],
            par_w: vec![0; n],
            root_w: vec![0; n],
        };
        for e in es {
            let a = (e[0] - 1) as usize;
            let b = (e[1] - 1) as usize;
            let w = e[2];
            this.g[a].push((b, w));
            this.g[b].push((a, w));
        }
        this.prepare();
        this
    }

    fn prepare(&mut self) {
        let mut timer = 0;
        self.dfs_prepare(0, &mut timer);
    }

    fn dfs_prepare(&mut self, v: usize, timer: &mut usize) {
        self.tin[v] = *timer;
        *timer += 1;

        for i in 0..self.g[v].len() {
            let (to, w) = self.g[v][i];
            if to == self.par[v] {
                continue;
            }
            self.par[to] = v;
            self.par_w[to] = w;
            self.root_w[to] = self.root_w[v] + w;
            self.dfs_prepare(to, timer);
        }

        self.tout[v] = *timer;
    }
}

impl Solution {
    pub fn tree_queries(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut tree = Tree::new(&edges);

        let mut segm_tree = SegmTreeSum::new(n);
        for i in 0..n {
            segm_tree.run_add_seg(tree.tin[i], tree.tin[i] + 1, tree.root_w[i] as i64);
        }

        let mut ans = Vec::new();
        for q in &queries {
            let t = q[0];
            match t {
                1 => {
                    let mut a = (q[1] - 1) as usize;
                    let mut b = (q[2] - 1) as usize;
                    let w = q[3];
                    if tree.par[a] == b {
                        std::mem::swap(&mut a, &mut b);
                    }
                    assert!(tree.par[b] == a);
                    let diff = -tree.par_w[b] + w;
                    tree.par_w[b] = w;
                    segm_tree.run_add_seg(tree.tin[b], tree.tout[b], diff as i64);
                }
                2 => {
                    let x = (q[1] - 1) as usize;
                    let cur = segm_tree.run_get_sum(tree.tin[x], tree.tin[x] + 1);
                    ans.push(cur as i32);
                }
                _ => panic!(),
            }
        }
        ans
    }
}

// SUM AND ADD ON SEGMENT
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Node {
    sum: i64,
    to_add: i64,
    len: usize,
}

impl Node {
    fn new(val: i64, len: usize) -> Self {
        Self {
            sum: val,
            to_add: 0,
            len,
        }
    }

    fn merge(v0: &Node, v1: &Node) -> Node {
        Self {
            sum: v0.sum + v1.sum,
            to_add: 0,
            len: v0.len + v1.len,
        }
    }

    fn apply(&self, to_add: i64) -> Self {
        Self {
            sum: self.sum + to_add * self.len as i64,
            to_add: self.to_add + to_add,
            len: self.len,
        }
    }

    fn push_to(&self, v: &Self) -> Self {
        v.apply(self.to_add)
    }

    fn remove_push_data(&mut self) {
        self.to_add = 0;
    }
}

struct SegmTreeSum {
    t: Vec<Node>,
    sz: usize,
}

impl SegmTreeSum {
    fn new(n: usize) -> Self {
        let mut this = Self {
            t: Vec::new(),
            sz: 1,
        };
        while this.sz < n {
            this.sz *= 2;
        }
        this.t = vec![Node::new(0, 0); this.sz * 2];
        for v in this.sz..this.sz + n {
            this.t[v] = Node::new(0, 1);
        }
        for v in (1..this.sz).rev() {
            this.upd(v);
        }
        this
    }

    fn upd(&mut self, v: usize) {
        assert!(self.t[v].to_add == 0);
        self.t[v] = Node::merge(&self.t[v * 2], &self.t[v * 2 + 1])
    }

    fn push(&mut self, v: usize) {
        self.t[v * 2] = Node::push_to(&self.t[v], &self.t[v * 2]);
        self.t[v * 2 + 1] = Node::push_to(&self.t[v], &self.t[v * 2 + 1]);
        self.t[v].remove_push_data();
    }

    // [l; r)
    fn run_add_seg(&mut self, l: usize, r: usize, to_add: i64) {
        self.add_seg(1, 0, self.sz, l, r, to_add)
    }

    fn add_seg(&mut self, v: usize, tl: usize, tr: usize, mut l: usize, mut r: usize, to_add: i64) {
        l = l.max(tl);
        r = r.min(tr);
        if l >= r {
            return;
        }

        if l == tl && r == tr {
            self.t[v] = self.t[v].apply(to_add);
            return;
        }

        self.push(v);

        let tm = (tl + tr) / 2;
        self.add_seg(v * 2, tl, tm, l, r, to_add);
        self.add_seg(v * 2 + 1, tm, tr, l, r, to_add);

        self.upd(v);
    }

    fn run_get_sum(&mut self, l: usize, r: usize) -> i64 {
        self.get_sum(1, 0, self.sz, l, r)
    }

    fn get_sum(&mut self, v: usize, tl: usize, tr: usize, mut l: usize, mut r: usize) -> i64 {
        l = l.max(tl);
        r = r.min(tr);
        if l >= r {
            return 0;
        }

        if l == tl && r == tr {
            return self.t[v].sum;
        }

        self.push(v);

        let tm = (tl + tr) / 2;
        let mut ans = 0;
        ans += self.get_sum(v * 2, tl, tm, l, r);
        ans += self.get_sum(v * 2 + 1, tm, tr, l, r);

        self.upd(v);

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
    #[case(2, vec![vec![1,2,7]], vec![vec![2,2],vec![1,1,2,4],vec![2,2]], vec![7,4])]
    #[case(3, vec![vec![1,2,2],vec![1,3,4]], vec![vec![2,1],vec![2,3],vec![1,1,3,7],vec![2,2],vec![2,3]], vec![0,4,2,7])]
    #[case(4, vec![vec![1,2,2],vec![2,3,1],vec![3,4,5]], vec![vec![2,4],vec![2,3],vec![1,2,3,3],vec![2,2],vec![2,3]], vec![8,3,2,5])]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] queries: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::tree_queries(n, edges, queries);
        assert_eq!(actual, expected);
    }
}
