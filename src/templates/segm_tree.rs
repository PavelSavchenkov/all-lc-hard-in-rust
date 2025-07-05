// MAX ON SEGMENT
struct SegmTreeMax {
    t: Vec<i64>,
    sz: usize,
}

impl SegmTreeMax {
    fn new(n: usize, val: i64) -> Self {
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        Self {
            sz,
            t: vec![val; sz * 2],
        }
    }

    fn assign_point(&mut self, pos: usize, val: i64) {
        let mut v = self.sz + pos;
        self.t[v] = val;
        v /= 2;
        while v >= 1 {
            self.t[v] = self.t[v * 2].max(self.t[v * 2 + 1]);
            v /= 2;
        }
    }

    // [l; r)
    fn get_max(&self, mut l: usize, mut r: usize) -> i64 {
        let mut mx = i64::MIN;
        if l >= r {
            return mx
        }
        r -= 1;
        l += self.sz;
        r += self.sz;
        while l <= r {
            mx = mx.max(self.t[l]);
            mx = mx.max(self.t[r]);
            l = (l + 1) / 2;
            r = (r - 1) / 2;
        }
        mx
    }
}

// BINARY FLIP ON SEGMENT, SUM ON SEGMENT
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
            flip: false
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
            return *v
        }
        v.do_flip()
    }
    
    fn remove_push_data(&mut self) {
        self.flip = false;
    }
}

struct SegmTreeFlip {
    t: Vec<Node>,
    sz: usize,
}

impl SegmTreeFlip {
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
            return
        }

        if l == tl && r == tr {
            self.t[v] = self.t[v].do_flip();
            return
        }

        self.push(v);

        let tm = (tl + tr) / 2;
        self.do_flip_t(v * 2, tl, tm, l, r);
        self.do_flip_t(v * 2 + 1, tm, tr, l, r);

        self.upd(v);
    }
}

// SUM AND ADD ON SEGMENT
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Node {
    sum: u64,
    to_add: u64,
    len: usize,
}

impl Node {
    fn new(val: u64, len: usize) -> Self {
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

    fn apply(&self, to_add: u64) -> Self {
        Self {
            sum: self.sum + to_add * self.len as u64,
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
    fn run_add_seg(&mut self, l: usize, r: usize, to_add: u64) {
        self.add_seg(1, 0, self.sz, l, r, to_add)
    }

    fn add_seg(&mut self, v: usize, tl: usize, tr: usize, mut l: usize, mut r: usize, to_add: u64) {
        l = l.max(tl);
        r = r.min(tr);
        if l >= r {
            return
        }

        if l == tl && r == tr {
            self.t[v] = self.t[v].apply(to_add);
            return
        }

        self.push(v);

        let tm = (tl + tr) / 2;
        self.add_seg(v * 2, tl, tm, l, r, to_add);
        self.add_seg(v * 2 + 1, tm, tr, l, r, to_add);

        self.upd(v);
    }

    fn run_get_sum(&mut self, l: usize, r: usize) -> u64 {
        self.get_sum(1, 0, self.sz, l, r)
    }

    fn get_sum(&mut self, v: usize, tl: usize, tr: usize, mut l: usize, mut r: usize) -> u64 {
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
