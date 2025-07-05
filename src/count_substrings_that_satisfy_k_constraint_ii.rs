//! Solution for https://leetcode.com/problems/count-substrings-that-satisfy-k-constraint-ii
//! 3261. Count Substrings That Satisfy K-Constraint II

struct SegmTree {
    sum: Vec<i64>,
    to_add: Vec<i64>,
    sz: usize,
}

impl SegmTree {
    fn new(n: usize) -> Self {
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        let sum = vec![0; sz * 2];
        let to_add = vec![0; sz * 2];
        Self { sum, to_add, sz }
    }

    fn do_add(&mut self, v: usize, len: usize, x: i64) {
        self.to_add[v] += x;
        self.sum[v] += len as i64 * x;
    }

    fn push(&mut self, v: usize, len: usize) {
        self.do_add(v * 2, len / 2, self.to_add[v]);
        self.do_add(v * 2 + 1, len / 2, self.to_add[v]);
        self.to_add[v] = 0;
    }

    fn upd(&mut self, v: usize) {
        assert!(self.to_add[v] == 0);
        self.sum[v] = self.sum[v * 2] + self.sum[v * 2 + 1];
    }

    fn add_seg(&mut self, l: usize, r: usize, x: i64) {
        self.add_seg_rec(1, 0, self.sz, l, r, x);
    }

    fn add_seg_rec(&mut self, v: usize, tl: usize, tr: usize, mut l: usize, mut r: usize, x: i64) {
        l = l.max(tl);
        r = r.min(tr);
        if l >= r {
            return;
        }
        let len = tr - tl;
        if l == tl && r == tr {
            self.do_add(v, len, x);
            return;
        }
        self.push(v, len);
        let tm = (tl + tr) / 2;
        self.add_seg_rec(v * 2, tl, tm, l, r, x);
        self.add_seg_rec(v * 2 + 1, tm, tr, l, r, x);
        self.upd(v);
    }

    fn get_sum(&mut self, l: usize, r: usize) -> i64 {
        self.get_sum_rec(1, 0, self.sz, l, r)
    }

    fn get_sum_rec(&mut self, v: usize, tl: usize, tr: usize, mut l: usize, mut r: usize) -> i64 {
        l = l.max(tl);
        r = r.min(tr);
        if l >= r {
            return 0;
        }
        if l == tl && r == tr {
            return self.sum[v];
        }
        let len = tr - tl;
        self.push(v, len);
        let tm = (tl + tr) / 2;
        let mut ans = 0;
        ans += self.get_sum_rec(v * 2, tl, tm, l, r);
        ans += self.get_sum_rec(v * 2 + 1, tm, tr, l, r);
        self.upd(v);
        ans
    }
}

struct Query {
    l: usize,
    r: usize,
    id: usize,
}

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let s = to_u8(&s);
        let n = s.len();
        let k = k as usize;

        let mut queries: Vec<_> = (0..queries.len())
            .map(|i| Query {
                l: queries[i][0] as usize,
                r: queries[i][1] as usize,
                id: i,
            })
            .collect();
        queries.sort_by_key(|q| q.r);

        let mut ptr = 0;
        let mut tree = SegmTree::new(n);
        let mut ans = vec![0; queries.len()];
        let mut positions = vec![Vec::new(); 2];
        for i in 0..n {
            let x = (s[i] - b'0') as usize;
            assert!(x <= 1);
            positions[x].push(i);

            let mut l = i;
            for x in 0..2 {
                let cnt = positions[x].len();
                if cnt >= k + 1 {
                    l = l.min(positions[x][cnt - k - 1] + 1);
                } else {
                    l = 0;
                }
            }
            tree.add_seg(l, i + 1, 1);

            while ptr < queries.len() && queries[ptr].r <= i {
                if queries[ptr].r == i {
                    let cur = tree.get_sum(queries[ptr].l, i + 1);
                    ans[queries[ptr].id] = cur;
                }
                ptr += 1;
            }
        }
        ans
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("0001111", 2, vec![vec![0,6]], vec![26])]
    #[case("010101", 1, vec![vec![0,5],vec![1,4],vec![2,3]], vec![15,9,3])]
    fn case(
        #[case] s: String,
        #[case] k: i32,
        #[case] queries: Vec<Vec<i32>>,
        #[case] expected: Vec<i64>,
    ) {
        let actual = Solution::count_k_constraint_substrings(s, k, queries);
        assert_eq!(actual, expected);
    }
}
