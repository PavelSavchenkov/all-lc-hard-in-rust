//! Solution for https://leetcode.com/problems/alternating-groups-iii
//! 3245. Alternating Groups III

struct SegmTreeSum {
    a0: Vec<i64>,
    d: Vec<i64>,
    sz: usize,
}

impl SegmTreeSum {
    fn new(n: usize) -> Self {
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        Self {
            a0: vec![0; sz * 2],
            d: vec![0; sz * 2],
            sz,
        }
    }

    fn get(&self, pos: usize) -> i64 {
        self.get_rec(1, 0, self.sz, pos)
    }

    fn get_rec(&self, v: usize, tl: usize, tr: usize, pos: usize) -> i64 {
        if pos < tl || pos >= tr {
            return 0;
        }

        let mut ans = self.a0[v] + self.d[v] * (pos - tl) as i64;
        if tl + 1 < tr {
            let tm = (tl + tr) / 2;
            ans += self.get_rec(v * 2, tl, tm, pos);
            ans += self.get_rec(v * 2 + 1, tm, tr, pos);
        }
        ans
    }

    fn add_seg(&mut self, l: usize, r: usize, a0: i64, d: i64) {
        // eprintln!("add seg: l={}, r={}, a0={}, d={}", l, r, a0, d);
        self.add_seg_rec(1, 0, self.sz, l, r, a0, d);
    }

    fn add_seg_rec(
        &mut self,
        v: usize,
        tl: usize,
        tr: usize,
        mut l: usize,
        mut r: usize,
        mut a0: i64,
        d: i64,
    ) {
        if l < tl {
            a0 += d * (tl - l) as i64;
            l = tl;
        }
        r = r.min(tr);
        if l >= r {
            return;
        }

        if l == tl && r == tr {
            self.a0[v] += a0;
            self.d[v] += d;
            return;
        } 

        let tm = (tl + tr) / 2;
        self.add_seg_rec(v * 2, tl, tm, l, r, a0, d);
        if l < tm {
            a0 += d * (tm - l) as i64;
            l = tm;
        }
        self.add_seg_rec(v * 2 + 1, tm, tr, l, r, a0, d);
    }
}

use std::collections::BTreeSet;
use std::ops::Bound::{Unbounded, Excluded};

struct Solver {
    tree: SegmTreeSum,
    a: Vec<i32>, // 0/1
    n: usize,
    segs: BTreeSet<(usize, usize)>,
}

impl Solver {
    fn new(n: usize) -> Self {
        Self {
            tree: SegmTreeSum::new(n + 1),
            a: vec![0; n],
            n,
            segs: BTreeSet::new(),
        }
    }

    fn update_seg(&mut self, l: usize, r: usize, coef: i32) {
        // eprintln!("update seg: l={}, r={}, coef={}", l, r, coef);
        let len = r - l;
        // func[len] += 1
        // func[len - 1] += 2
        // ...
        // func[1] += len
        let a0 = (len as i64) * (coef as i64);
        let d = (-1) * (coef as i64);
        self.tree.add_seg(1, len + 1, a0, d);
        match coef {
            1 => {
                self.segs.insert((l, r));
            },
            -1 => {
                self.segs.remove(&(l, r));
            },
            _ => panic!()
        }
    }

    fn flip(&mut self, pos: usize) {
        match self.a[pos] {
            1 => {
                // find seg and split
                let range = (Unbounded, Excluded((pos, usize::MAX)));
                let mut it = self.segs.range(range);
                let (l, r) = *it.next_back().unwrap();
                self.update_seg(l, r, -1);
                if l < pos {
                    self.update_seg(l, pos, 1);
                }
                if pos + 1 < r {
                    self.update_seg(pos + 1, r, 1);
                }
            },
            0 => {
                // merge left and right segs
                let mut seg = (pos, pos + 1);
                // 1. try left
                let range = (Unbounded, Excluded((pos, usize::MAX)));
                let mut it = self.segs.range(range);
                if let Some(&(l, r)) = it.next_back() {
                    if r == pos {
                        self.update_seg(l, r, -1);
                        seg.0 = l;
                    }
                }
                // 2. try right
                let range = (Excluded((pos, pos)), Unbounded);
                let mut it = self.segs.range(range);
                if let Some(&(l, r)) = it.next() {
                    if l == pos + 1 {
                        self.update_seg(l, r, -1);
                        seg.1 = r;
                    }
                }
                // add newly formed seg
                self.update_seg(seg.0, seg.1, 1);
            },
            _ => panic!()
        }
        self.a[pos] ^= 1;
    }

    fn ask(&self, sz: usize) -> usize {
        let ans = self.tree.get(sz);
        assert!(ans >= 0);
        let mut ans = ans as usize;
        // add new segments with overlap with array end
        let mut suff = 0;
        let mut pref = 0;
        if !self.segs.is_empty() {
            let &(l, r) = self.segs.last().unwrap();
            if r == self.n {
                suff = r - l;
            }
            let &(l, r) = self.segs.first().unwrap();
            if l == 0 {
                pref = r - l;
            }
        }
        // entire array is 1s
        if pref + suff >= self.n {
            return self.n;
        }
        if pref > 0 && suff > 0 {
            let first = (self.n - suff).max(self.n - (sz - 1));
            let mut last = self.n - 1;
            if pref < sz - 1 {
                last = self.n - 1 - ((sz - 1) - pref);
            }
            if first <= last {
                ans += last - first + 1;
            }
        }
        // eprintln!("ask sz={}, ans={}", sz, ans);
        ans
    }
}

impl Solution {
    pub fn number_of_alternating_groups(mut c: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = c.len();

        let mut solver = Solver::new(n);
        for i in 0..n {
            if c[i] != c[(i + 1) % n] {
                solver.flip(i); 
            }
        }

        let mut ans = Vec::new();
        for q in &queries {
            match q[0] {
                1 => {
                    let sz = q[1] as usize;
                    assert!(sz > 1);
                    ans.push(solver.ask(sz - 1) as i32);
                },
                2 => {
                    let i = q[1] as usize;
                    let color = q[2];
                    if c[i] != color {
                        solver.flip(i);
                        solver.flip((i + n - 1) % n);
                        c[i] = color;
                    }
                },
                _ => panic!()
            }
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
    #[case(vec![0,1,1,0,1], vec![vec![2,1,0],vec![1,4]], vec![2])]
    #[case(vec![0,0,1,0,1,1], vec![vec![1,3],vec![2,3,0],vec![1,5]], vec![2,0])]
    fn case(#[case] colors: Vec<i32>, #[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::number_of_alternating_groups(colors, queries);
        assert_eq!(actual, expected);
    }
}
