//! Solution for https://leetcode.com/problems/tiling-a-rectangle-with-the-fewest-squares
//! 1240. Tiling a Rectangle with the Fewest Squares

use std::collections::HashMap;

struct Tiling {
    height: Vec<usize>,
    rows: usize,
    cols: usize,
    hash: u64,
    coef_col: Vec<u64>,
    history: Vec<(usize, usize, usize)>,
}

impl Tiling {
    fn new(n: usize, m: usize) -> Self {
        Self {
            height: vec![0; m],
            rows: n,
            cols: m,
            hash: 0,
            coef_col: (0..m).map(|i| u64::pow(n as u64 + 1, i as u32)).collect(),
            history: Vec::new(),
        }
    }

    fn put(&mut self, l: usize, r: usize, height: usize) {
        let height0 = self.height[l];
        self.history.push((l, r, height));
        for i in l..r {
            assert!(self.height[i] == height0);
            self.height[i] += height;
            assert!(self.height[i] <= self.rows);
            self.hash += height as u64 * self.coef_col[i];
        }
    }

    fn rollback(&mut self) {
        let (l, r, height) = self.history.pop().unwrap();
        for i in l..r {
            self.height[i] -= height;
            self.hash -= height as u64 * self.coef_col[i];
        }
    }

    fn get_hash(&self) -> u64 {
        // return get_hash(&self.height);
        self.hash
    }

    fn size(&self) -> usize {
        self.history.len()
    }

    fn done(&self) -> bool {
        *self.height.iter().min().unwrap() as usize == self.rows
    }

    fn seg_min(&self) -> (usize, usize) {
        let min = *self.height.iter().min().unwrap();
        let l = self.height.iter().position(|&h| h == min).unwrap();
        let mut r = l;
        while r < self.height.len() && self.height[r] == self.height[l] {
            r += 1;
        }
        (l, r)
    }

    fn print(&self) {
        for &h in &self.height {
            eprint!("{} ", h);
        }
        eprintln!("history:");
        for (l, r, h) in &self.history {
            eprintln!("{} {} {}", l, r, h);
        }
    }
}

fn go(tiling: &mut Tiling, dp: &mut HashMap<u64, usize>, ans: &mut usize) -> usize {
    let mem = dp.get(&tiling.get_hash());
    if mem.is_some() {
        let remaining = *mem.unwrap();
        if remaining < usize::MAX {
            *ans = (*ans).min(tiling.size() + remaining);
            // tiling.print();
            // eprintln!("remaining = {}", remaining);
        }
        return remaining;
    }

    if tiling.done() {
        *ans = (*ans).min(tiling.size());
        // tiling.print();
        return 0;
    }
    if tiling.size() + 1 >= *ans {
        return usize::MAX;
    }

    let (l, r) = tiling.seg_min();
    let max_h = (r - l).min(tiling.rows - tiling.height[l]);
    let mut best_remaining = usize::MAX;
    for h in (1..=max_h).rev() {
        for i in l..r - h + 1 {
            tiling.put(i, i + h, h);
            let remaining = go(tiling, dp, ans);
            if remaining < usize::MAX {
                best_remaining = best_remaining.min(remaining + 1);
            }
            tiling.rollback();
        }
    }

    dp.insert(tiling.get_hash(), best_remaining);
    best_remaining
}

impl Solution {
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;

        let mut tiling = Tiling::new(n, m);
        let mut dp = HashMap::new();
        let mut ans = usize::MAX;
        go(&mut tiling, &mut dp, &mut ans);
        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 3, 3)]
    #[case(5, 8, 5)]
    #[case(11, 13, 6)]
    fn case(#[case] n: i32, #[case] m: i32, #[case] expected: i32) {
        let actual = Solution::tiling_rectangle(n, m);
        assert_eq!(actual, expected);
    }
}
