//! Solution for https://leetcode.com/problems/block-placement-queries
//! 3161. Block Placement Queries

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included, Unbounded};

struct FenwTreeMax {
    t: Vec<u32>,
}

impl FenwTreeMax {
    fn new(n: usize) -> Self {
        Self { t: vec![0; n] }
    }

    // [0; r)
    fn get_max(&self, mut r: usize) -> u32 {
        if r == 0 {
            return 0;
        }
        r -= 1;
        let mut ans = 0;
        loop {
            ans = ans.max(self.t[r]);
            r &= r + 1;
            if r == 0 {
                break;
            }
            r -= 1;
        }
        ans
    }

    fn update(&mut self, pos: usize, val: u32) {
        let mut i = pos;
        while i < self.t.len() {
            self.t[i] = self.t[i].max(val);
            i |= i + 1;
        }
    }
}

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut xs = Vec::new();
        xs.push(0);
        for q in &queries {
            xs.push(q[1]);
        }
        xs.sort();
        let mx = xs.last().unwrap();
        xs.push(mx + 1);
        xs.dedup();

        let mut alive_xs = BTreeSet::new();
        alive_xs.insert(0);
        alive_xs.insert(xs.len() - 1);

        let mut tree = FenwTreeMax::new(xs.len());
        for q in &queries {
            if q[0] == 1 {
                let x = q[1];
                let id = xs.binary_search(&x).unwrap();
                alive_xs.insert(id);
            }
        }
        let mut prev = None;
        for i in 0..xs.len() {
            if alive_xs.contains(&i) {
                if let Some(pos) = prev {
                    let dist = xs[i] - xs[pos];
                    tree.update(pos, dist as u32);
                    eprintln!("pos = {}, xs[pos] = {}, dist = {}", pos, xs[pos], dist);
                }
                prev = Some(i);
            }
        }

        let mut ans = Vec::new();
        for q in queries.iter().rev() {
            if q[0] == 1 {
                let x = q[1];
                let id = xs.binary_search(&x).unwrap();

                let range = (Excluded(id), Unbounded);
                let mut it = alive_xs.range(range);
                let next = *it.next().unwrap();

                let range = (Unbounded, Excluded(id));
                let mut it = alive_xs.range(range);
                let prev = *it.next_back().unwrap();

                let dist = xs[next] - xs[prev];
                tree.update(prev, dist as u32);

                alive_xs.remove(&id);
            } else {
                let x = q[1];
                let id = xs.binary_search(&x).unwrap();
                let sz = q[2];

                let mut fit = false;

                let range = (Unbounded, Included(id));
                let mut it = alive_xs.range(range);
                let last = *it.next_back().unwrap();
                // eprintln!(
                //     "x = {}, sz = {}, last = {}, xs[last] = {}, id = {}",
                //     x, sz, last, xs[last], id
                // );
                if x - xs[last] >= sz {
                    fit = true;
                } else if last > 0 {
                    let mx = tree.get_max(last);
                    if mx >= sz as u32 {
                        fit = true;
                    }
                }
                ans.push(fit);
            }
        }

        ans.reverse();
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
    #[case(vec![vec![1,2],vec![2,3,3],vec![2,3,1],vec![2,2,2]], vec![false,true,true])]
    #[case(vec![vec![1,7],vec![2,7,6],vec![1,2],vec![2,7,5],vec![2,7,6]], vec![true,true,false])]
    fn case(#[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<bool>) {
        let actual = Solution::get_results(queries);
        assert_eq!(actual, expected);
    }
}
