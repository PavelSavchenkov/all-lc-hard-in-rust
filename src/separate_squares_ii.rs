//! Solution for https://leetcode.com/problems/separate-squares-ii
//! 3454. Separate Squares II

#[derive(Clone, Copy)]
struct Node {
    to_add: i32,
    min: i32,
    sum_coef_min: i64,
}

impl Node {
    fn new(coef: i64, val: i32) -> Self {
        Self {
            to_add: 0,
            min: val,
            sum_coef_min: coef,
        }
    }

    fn apply_add(&mut self, add: i32) {
        self.to_add += add;
        self.min += add;
    }

    fn merge(&self, other: &Self) -> Self {
        let min = self.min.min(other.min);
        let mut sum_coef_min = 0;
        if self.min == min {
            sum_coef_min += self.sum_coef_min;
        }
        if other.min == min {
            sum_coef_min += other.sum_coef_min;
        }
        Self {
            to_add: 0,
            min,
            sum_coef_min,
        }
    }
}

struct SegmTree {
    t: Vec<Node>,
    sz: usize,
}

impl SegmTree {
    fn new(coefs: &Vec<i64>) -> Self {
        let n = coefs.len();
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        let mut t = vec![Node::new(0, 0); sz * 2];
        for i in 0..n {
            t[sz + i] = Node::new(coefs[i], 0);
        }
        let mut this = Self { t, sz };
        for v in (1..sz).rev() {
            this.update(v);
        }
        this
    }

    fn update(&mut self, v: usize) {
        self.t[v] = self.t[v * 2].merge(&self.t[v * 2 + 1]);
    }

    fn add_seg(&mut self, l: usize, r: usize, val: i32) {
        self.add_seg_rec(1, 0, self.sz, l, r, val);
    }

    fn push(&mut self, v: usize) {
        let to_add = self.t[v].to_add;
        if to_add == 0 {
            return;
        }
        self.t[v * 2].apply_add(to_add);
        self.t[v * 2 + 1].apply_add(to_add);
        self.t[v].to_add = 0;
    }

    fn add_seg_rec(
        &mut self,
        v: usize,
        tl: usize,
        tr: usize,
        mut l: usize,
        mut r: usize,
        val: i32,
    ) {
        l = l.max(tl);
        r = r.min(tr);
        if l >= r {
            return;
        }
        if l == tl && r == tr {
            self.t[v].apply_add(val);
            return;
        }
        self.push(v);
        let tm = (tl + tr) / 2;
        self.add_seg_rec(v * 2, tl, tm, l, r, val);
        self.add_seg_rec(v * 2 + 1, tm, tr, l, r, val);
        self.update(v);
    }

    fn get_node(&mut self, l: usize, r: usize) -> Node {
        self.get_node_rec(1, 0, self.sz, l, r)
    }

    fn get_node_rec(&mut self, v: usize, tl: usize, tr: usize, mut l: usize, mut r: usize) -> Node {
        l = l.max(tl);
        r = r.min(tr);
        if l >= r {
            return Node::new(0, i32::MAX);
        }
        if l == tl && r == tr {
            return self.t[v];
        }
        self.push(v);
        let tm = (tl + tr) / 2;
        let left = self.get_node_rec(v * 2, tl, tm, l, r);
        let right = self.get_node_rec(v * 2 + 1, tm, tr, l, r);
        self.update(v);
        left.merge(&right)
    }
}

#[derive(Clone, Copy)]
struct Square {
    x: i32,
    y: i32,
    len: i32,
}
struct Event {
    x: usize,
    y0: usize,
    y1: usize,
    val: i32, // +- 1
}

fn calc_left(squares: &Vec<Square>) -> (Vec<i64>, Vec<i32>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for s in squares {
        xs.push(s.x);
        xs.push(s.x + s.len);
        ys.push(s.y);
        ys.push(s.y + s.len);
    }
    xs.sort();
    xs.dedup();
    ys.sort();
    ys.dedup();

    let mut coefs_y = Vec::new();
    for i in 0..ys.len() - 1 {
        let dy = ys[i + 1] - ys[i];
        coefs_y.push(dy as i64);
    }

    let mut tree = SegmTree::new(&coefs_y);
    let mut es = Vec::new();
    for s in squares {
        let x0 = xs.binary_search(&s.x).unwrap();
        let x1 = xs.binary_search(&(s.x + s.len)).unwrap();
        let y0 = ys.binary_search(&s.y).unwrap();
        let y1 = ys.binary_search(&(s.y + s.len)).unwrap();

        es.push(Event {
            x: x0,
            y0,
            y1,
            val: 1,
        });
        es.push(Event {
            x: x1,
            y0,
            y1,
            val: -1,
        });
    }
    es.sort_by_key(|e| e.x);

    let mut pref_area = 0;
    let mut ans = vec![0; xs.len()];
    let mut i = 0;
    while i < es.len() {
        let mut j = i;
        while j < es.len() && es[i].x == es[j].x {
            j += 1;
        }
        let x = es[i].x;

        let node = tree.get_node(0, ys.len() - 1);
        let mut ver_len = (*ys.last().unwrap() - ys[0]) as i64;
        if node.min == 0 {
            ver_len -= node.sum_coef_min;
        }
        let mut dx = 0;
        if x > 0 {
            dx = (xs[x] - xs[x - 1]) as i64;
        }
        pref_area += dx * ver_len;
        ans[x] = pref_area;

        for k in i..j {
            tree.add_seg(es[k].y0, es[k].y1, es[k].val);
        }
        i = j;
    }

    (ans, xs)
}

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let squares: Vec<_> = squares
            .iter()
            .map(|v| Square {
                x: v[1],
                y: v[0],
                len: v[2],
            })
            .collect();

        let (left_area, xs) = calc_left(&squares);
        let total_area = left_area.last().unwrap();
        eprintln!("total_area={}", total_area);
        for i in 1..xs.len() {
            let left = left_area[i];
            let right = total_area - left;
            if left >= right {
                let dx = (xs[i] - xs[i - 1]) as i64;
                let mut ver_len = left - left_area[i - 1];
                assert!(ver_len % dx == 0);
                ver_len /= dx;
                let left = left_area[i - 1];
                let x0 = xs[i - 1] as f64;
                let x1 = xs[i] as f64;
                let x = (x1 + x0 + (right - left) as f64 / ver_len as f64) * 0.5;
                return x;
            }
        }
        unreachable!()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,0,1],vec![2,2,1]], 1.00000)]
    #[case(vec![vec![0,0,2],vec![1,1,1]], 1.00000)]
    #[case(vec![vec![15,21,2],vec![19,21,3]], 22.30000)]
    #[case(vec![vec![26,29,3],vec![10,24,1]], 30.3333333)]
    fn case(#[case] squares: Vec<Vec<i32>>, #[case] expected: f64) {
        let actual = Solution::separate_squares(squares);
        assert!(
            (actual - expected).abs() < 1e-5,
            "Assertion failed: actual {actual:.5} but expected {expected:.5}. Diff is more than 1e-5."
        );
    }
}
