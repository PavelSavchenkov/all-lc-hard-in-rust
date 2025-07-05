//! Solution for https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet
//! 2940. Find Building Where Alice and Bob Can Meet

use std::collections::BTreeSet;
use std::ops::Bound::{Included, Unbounded};

struct Query {
    a: usize,
    b: usize,
    id: usize,
}

impl Query {
    fn new(v: &Vec<i32>, id: usize) -> Self {
        let mut a = v[0] as usize;
        let mut b = v[1] as usize;
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        Self { a, b, id }
    }
}

impl Solution {
    pub fn leftmost_building_queries(a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = a.len();
        let q = queries.len();

        let mut queries: Vec<_> = (0..q).map(|i| Query::new(&queries[i], i)).collect();

        queries.sort_by_key(|q| -(a[q.a].max(a[q.b])));

        let mut by_h: Vec<_> = (0..n).collect();
        by_h.sort_by_key(|&i| -a[i]);

        let mut set = BTreeSet::new();
        let mut ptr = 0;
        let mut ans = vec![-1; q];
        for q in &queries {
            let h = a[q.a].max(a[q.b]);
            while ptr < n && a[by_h[ptr]] > h {
                set.insert(by_h[ptr]);
                ptr += 1;
            }

            if a[q.a] < a[q.b] || q.a == q.b {
                ans[q.id] = q.b as i32;
            } else {
                let left = q.a.max(q.b);
                let range = (Included(left), Unbounded);
                let mut it = set.range(range);
                if let Some(&pos) = it.next() {
                    ans[q.id] = pos as i32;
                }
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
    #[case(vec![6,4,8,5,2,7], vec![vec![0,1],vec![0,3],vec![2,4],vec![3,4],vec![2,2]], vec![2,5,-1,5,2])]
    #[case(vec![5,3,8,2,6,1,4,6], vec![vec![0,7],vec![3,5],vec![5,2],vec![3,0],vec![1,6]], vec![7,6,-1,4,6])]
    fn case(#[case] heights: Vec<i32>, #[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::leftmost_building_queries(heights, queries);
        assert_eq!(actual, expected);
    }
}
