//! Solution for https://leetcode.com/problems/find-x-value-of-array-ii
//! 3525. Find X Value of Array II

#[derive(Clone)]
struct Node {
    prefs: Vec<usize>,
    rem: usize,
}

impl Node {
    fn new(val: usize, k: usize) -> Self {
        assert!(val < k);
        let mut prefs = vec![0; k];
        prefs[val] = 1;
        Self { prefs, rem: val }
    }

    fn empty(k: usize) -> Self {
        Self {
            prefs: vec![0; k],
            rem: k,
        }
    }

    fn is_empty(&self) -> bool {
        self.rem == self.prefs.len()
    }

    fn merge(&self, other: &Self) -> Self {
        if self.is_empty() {
            return other.clone();
        }
        if other.is_empty() {
            return self.clone();
        }
        let k = self.prefs.len();
        let rem = (self.rem * other.rem) % k;
        let mut prefs = self.prefs.clone();
        for r in 0..k {
            prefs[(self.rem * r) % k] += other.prefs[r];
        }
        Self { prefs, rem }
    }
}

struct SegmTree {
    t: Vec<Node>,
    sz: usize,
    k: usize,
}

impl SegmTree {
    fn new(n: usize, k: usize) -> Self {
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        Self {
            t: vec![Node::empty(k); sz * 2],
            sz,
            k,
        }
    }

    // [l; r)
    fn get(&self, l: usize, r: usize) -> Node {
        self.get_rec(1, 0, self.sz, l, r)
    }

    fn get_rec(&self, v: usize, tl: usize, tr: usize, mut l: usize, mut r: usize) -> Node {
        l = l.max(tl);
        r = r.min(tr);
        if l >= r {
            return Node::empty(self.k);
        }
        if l == tl && r == tr {
            return self.t[v].clone();
        }
        let tm = (tl + tr) / 2;
        let left = self.get_rec(v * 2, tl, tm, l, r);
        let right = self.get_rec(v * 2 + 1, tm, tr, l, r);
        left.merge(&right)
    }

    fn update(&mut self, pos: usize, val: usize) {
        let mut v = self.sz + pos;
        self.t[v] = Node::new(val, self.k);
        v /= 2;
        while v > 0 {
            self.t[v] = self.t[v * 2].merge(&self.t[v * 2 + 1]);
            v /= 2;
        }
    }
}

impl Solution {
    pub fn result_array(a: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|x| (x % k) as usize).collect();

        let mut tree = SegmTree::new(n, k as usize);
        for i in 0..n {
            tree.update(i, a[i]);
        }

        let mut ans = Vec::new();
        for q in &queries {
            let i = q[0] as usize;
            let val = (q[1] % k) as usize;
            let start = q[2] as usize;
            let x = q[3] as usize;

            tree.update(i, val);
            let node = tree.get(start, n);
            let cur = node.prefs[x];
            ans.push(cur as i32);
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
    #[case(vec![1,2,3,4,5], 3, vec![vec![2,2,0,2],vec![3,3,3,0],vec![0,1,0,1]], vec![2,2,2])]
    #[case(vec![1,2,4,8,16,32], 4, vec![vec![0,2,0,2],vec![0,2,0,1]], vec![1,0])]
    #[case(vec![1,1,2,1,1], 2, vec![vec![2,1,0,1]], vec![5])]
    fn case(
        #[case] nums: Vec<i32>,
        #[case] k: i32,
        #[case] queries: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::result_array(nums, k, queries);
        assert_eq!(actual, expected);
    }
}
