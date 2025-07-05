//! Solution for https://leetcode.com/problems/booking-concert-tickets-in-groups
//! 2286. Booking Concert Tickets in Groups

#[derive(Copy, Clone)]
struct Node {
    sum: u64,
    max: u32,
}

impl Node {
    fn new(val: u32) -> Self {
        Self {
            sum: val as u64,
            max: val,
        }
    }

    fn merge(&self, other: &Self) -> Self {
        Self {
            sum: self.sum + other.sum,
            max: self.max.max(other.max),
        }
    }
}

struct SegmTree {
    t: Vec<Node>,
    sz: usize,
}

impl SegmTree {
    fn new(n: usize, val: u32) -> Self {
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        Self {
            t: vec![Node::new(0); sz * 2],
            sz,
        }
    }

    // [l; r)
    fn get_sum(&self, mut l: usize, mut r: usize) -> u64 {
        l += self.sz;
        r += self.sz - 1;
        let mut sum = 0;
        while l <= r {
            if l % 2 == 1 {
                sum += self.t[l].sum;
            }
            if r % 2 == 0 {
                sum += self.t[r].sum;
            }
            l = (l + 1) / 2;
            r = (r - 1) / 2;
        }
        sum
    }

    fn update(&mut self, pos: usize, val: u32) {
        let mut v = self.sz + pos;
        self.t[v] = Node::new(val);
        v /= 2;
        while v > 0 {
            self.t[v] = self.t[v * 2].merge(&self.t[v * 2 + 1]);
            v /= 2;
        }
    }

    // [l; r)
    fn leftmost_at_least(&self, l: usize, r: usize, lower: u32) -> Option<usize> {
        self.leftmost_at_least_tree(1, 0, self.sz, l, r, lower)
    }

    fn leftmost_at_least_tree(
        &self,
        v: usize,
        tl: usize,
        tr: usize,
        mut l: usize,
        mut r: usize,
        lower: u32,
    ) -> Option<usize> {
        l = l.max(tl);
        r = r.min(tr);
        if l >= r || self.t[v].max < lower {
            return None;
        }

        if tl + 1 == tr {
            return Some(tl);
        }

        let tm = (tl + tr) / 2;
        let left = self.leftmost_at_least_tree(v * 2, tl, tm, l, r, lower);
        if left.is_some() {
            return left;
        }
        self.leftmost_at_least_tree(v * 2 + 1, tm, tr, l, r, lower)
    }
}

struct BookMyShow {
    t: SegmTree,
    ptr: usize,
    m: u32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        let n = n as usize;
        let m = m as u32;

        let mut this = Self {
            t: SegmTree::new(n, 0),
            ptr: 0,
            m,
        };
        for i in 0..n {
            this.t.update(i, m);
        }
        this
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let k = k as u32;
        let max_row = max_row as usize;

        let pos = self.t.leftmost_at_least(0, max_row + 1, k);
        if pos.is_none() {
            return Vec::new();
        }
        let pos = pos.unwrap();

        let free_now = self.t.get_sum(pos, pos + 1) as u32;
        assert!(free_now >= k);
        self.t.update(pos, free_now - k);
        vec![pos as i32, (self.m - free_now) as i32]
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        let mut k = k as u32;
        let max_row = max_row as usize;

        let sum_free = self.t.get_sum(0, max_row + 1);
        if sum_free < k as u64 {
            return false;
        }
        while k > 0 {
            let free_now = self.t.get_sum(self.ptr, self.ptr + 1) as u32;
            let take = free_now.min(k);
            let free_new = free_now - take;
            self.t.update(self.ptr, free_new);
            k -= take;
            if free_new == 0 {
                self.ptr += 1;
            }
        }
        assert!(k == 0);
        assert!(self.ptr <= max_row + 1);
        true
    }
}

/**
 * Your BookMyShow object will be instantiated and called as such:
 * let obj = BookMyShow::new(n, m);
 * let ret_1: Vec<i32> = obj.gather(k, maxRow);
 * let ret_2: bool = obj.scatter(k, maxRow);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
