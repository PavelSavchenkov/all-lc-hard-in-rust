//! Solution for https://leetcode.com/problems/shortest-distance-after-road-addition-queries-ii
//! 3244. Shortest Distance After Road Addition Queries II

use std::collections::BTreeSet;
use std::ops::Bound::Excluded;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut ends = BTreeSet::new();
        for i in 0..n {
            ends.insert(i);
        }

        let mut ans = Vec::new();
        for q in &queries {
            let a = q[0] as usize;
            let b = q[1] as usize;
            if ends.contains(&a) && ends.contains(&b) {
                let range = (Excluded(a), Excluded(b));
                loop {
                    let mut it = ends.range(range);
                    let i = it.next().copied();
                    if let Some(pos) = i {
                        assert!(a < pos && pos < b);
                        ends.remove(&pos);
                    } else {
                        break;
                    }
                }
            }
            ans.push((ends.len() - 1) as i32);
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
    #[case(5, vec![vec![2,4],vec![0,2],vec![0,4]], vec![3,2,1])]
    #[case(4, vec![vec![0,3],vec![0,2]], vec![1,1])]
    fn case(#[case] n: i32, #[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::shortest_distance_after_queries(n, queries);
        assert_eq!(actual, expected);
    }
}
