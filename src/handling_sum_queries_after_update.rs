//! Solution for https://leetcode.com/problems/handling-sum-queries-after-update
//! 2569. Handling Sum Queries After Update

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Node {
    cnt1: usize,
    len: usize,
    flip: bool,
}

impl Node {
    fn new(val: u32, len: usize) -> Self {
        assert!(val <= 1);
        Self {
            cnt1: val as usize,
            len,
            flip: false,
        }
    }

    fn merge(v0: &Node, v1: &Node) -> Node {
        Self {
            cnt1: v0.cnt1 + v1.cnt1,
            len: v0.len + v1.len,
            flip: false,
        }
    }

    fn do_flip(&self) -> Self {
        Self {
            cnt1: self.len - self.cnt1,
            len: self.len,
            flip: !self.flip,
        }
    }

    fn push(&self, v: &Self) -> Self {
        if !self.flip {
            return *v;
        }
        v.do_flip()
    }

    fn remove_push_data(&mut self) {
        self.flip = false;
    }
}

struct SegmTree {
    t: Vec<Node>,
    sz: usize,
}

impl SegmTree {
    fn new(n: usize) -> Self {
        let mut this = Self {
            t: Vec::new(),
            sz: 1,
        };
        while this.sz < n {
            this.sz *= 2;
        }
        this.t = vec![Node::new(0, 0); this.sz * 2];
        for i in 0..n {
            this.t[this.sz + i] = Node::new(0, 1);
        }
        for v in (1..this.sz).rev() {
            this.upd(v);
        }
        this
    }

    fn upd(&mut self, v: usize) {
        self.t[v] = Node::merge(&self.t[v * 2], &self.t[v * 2 + 1])
    }

    fn push(&mut self, v: usize) {
        self.t[v * 2] = Node::push(&self.t[v], &self.t[v * 2]);
        self.t[v * 2 + 1] = Node::push(&self.t[v], &self.t[v * 2 + 1]);
        self.t[v].remove_push_data();
    }

    fn get_cnt1(&mut self) -> usize {
        self.t[1].cnt1
    }

    fn do_flip(&mut self, l: usize, r: usize) {
        self.do_flip_t(1, 0, self.sz, l, r)
    }

    fn do_flip_t(&mut self, v: usize, tl: usize, tr: usize, mut l: usize, mut r: usize) {
        l = l.max(tl);
        r = r.min(tr);
        if l >= r {
            return;
        }

        if l == tl && r == tr {
            self.t[v] = self.t[v].do_flip();
            return;
        }

        self.push(v);

        let tm = (tl + tr) / 2;
        self.do_flip_t(v * 2, tl, tm, l, r);
        self.do_flip_t(v * 2 + 1, tm, tr, l, r);

        self.upd(v);
    }
}

impl Solution {
    pub fn handle_query(a: Vec<i32>, b: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let n = a.len();
        assert!(b.len() == n);

        let mut tree = SegmTree::new(n);
        for i in 0..n {
            if a[i] == 1 {
                tree.do_flip(i, i + 1);
            }
        }

        let mut ans: i64 = 0;
        for &x in &b {
            ans += x as i64;
        }

        let mut answers = Vec::new();
        for q in &queries {
            let t = q[0];
            match t {
                1 => {
                    let l = q[1] as usize;
                    let r = q[2] as usize;
                    tree.do_flip(l, r + 1);
                }
                2 => {
                    let p = q[1];
                    let cnt1 = tree.get_cnt1();
                    ans += p as i64 * cnt1 as i64;
                }
                3 => {
                    answers.push(ans);
                }
                _ => panic!("Wrong query"),
            }
        }

        answers
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,0,1], vec![0,0,0], vec![vec![1,1,1],vec![2,1,0],vec![3,0,0]], vec![3])]
    #[case(vec![1], vec![5], vec![vec![2,0,0],vec![3,0,0]], vec![5])]
    fn case(
        #[case] nums1: Vec<i32>,
        #[case] nums2: Vec<i32>,
        #[case] queries: Vec<Vec<i32>>,
        #[case] expected: Vec<i64>,
    ) {
        let actual = Solution::handle_query(nums1, nums2, queries);
        assert_eq!(actual, expected);
    }
}
