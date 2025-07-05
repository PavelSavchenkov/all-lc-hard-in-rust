//! Solution for https://leetcode.com/problems/sort-items-by-groups-respecting-dependencies
//! 1203. Sort Items by Groups Respecting Dependencies

use std::collections::BTreeSet;
use std::ops::Bound::Excluded;

impl Solution {
    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let m = m as usize;

        let mut g = vec![Vec::new(); n];
        for to in 0..n {
            for &from in &before_items[to] {
                g[from as usize].push(to);
            }
        }

        let mut deg_in_group = vec![0; m];
        let mut deg_in = vec![0; n];
        for v in 0..n {
            for &to in &g[v] {
                if group[to] != -1 && group[v] != group[to] {
                    deg_in_group[group[to] as usize] += 1;
                }
                deg_in[to] += 1;
            }
        }
        let mut good_groups = BTreeSet::<usize>::new();
        for g in 0..m {
            if deg_in_group[g] == 0 {
                good_groups.insert(g);
            }
        }
        let get_v_from_group = |g: i32, set: &BTreeSet<(usize, i32, usize)>| -> Option<usize> {
            let range = (Excluded((0, g - 1, n)), Excluded((0, g, n)));
            let mut it = set.range(range);
            if let Some((_, _, id)) = it.next() {
                return Some(*id);
            }
            None
        };

        let mut set = BTreeSet::new();
        for v in 0..n {
            set.insert((deg_in[v], group[v], v));
        }

        let mut ans = Vec::new();
        let mut prev_g = -1;
        for it in 0..n {
            let mut v = None;
            if prev_g != -1 {
                v = get_v_from_group(prev_g, &set);
            }
            if v.is_none() {
                v = get_v_from_group(-1, &set);
                if v.is_none() {
                    while !good_groups.is_empty() {
                        let g = *good_groups.first().unwrap();
                        v = get_v_from_group(g as i32, &set);
                        if v.is_some() {
                            break;
                        }
                        good_groups.remove(&g);
                    }
                }
            }
            if v.is_none() {
                return Vec::new();
            }
            let v = v.unwrap();
            set.remove(&(deg_in[v], group[v], v));
            prev_g = group[v];

            ans.push(v as i32);
            for &to in &g[v] {
                set.remove(&(deg_in[to], group[to], to));
                deg_in[to] -= 1;
                set.insert((deg_in[to], group[to], to));
                let g = group[to];
                if g != -1 && group[v] != g {
                    let g = g as usize;
                    deg_in_group[g] -= 1;
                    if deg_in_group[g] == 0 {
                        good_groups.insert(g);
                    }
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
    // #[case(8, 2, vec![-1,-1,1,0,0,1,0,-1], vec![vec![],vec![6],vec![5],vec![6],vec![3,6],vec![],vec![],vec![]], vec![6,3,4,1,5,2,0,7])]
    // #[case(8, 2, vec![-1,-1,1,0,0,1,0,-1], vec![vec![],vec![6],vec![5],vec![6],vec![3],vec![],vec![4],vec![]], vec![])]
    #[case(5, 5, vec![2,0,-1,3,0], vec![vec![2,1,3],vec![2,4],vec![],vec![],vec![]], vec![3,2,4,1,0])]
    fn case(
        #[case] n: i32,
        #[case] m: i32,
        #[case] group: Vec<i32>,
        #[case] before_items: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::sort_items(n, m, group, before_items);
        assert_eq!(actual, expected);
    }
}
