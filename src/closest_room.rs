//! Solution for https://leetcode.com/problems/closest-room
//! 1847. Closest Room

use std::collections::BTreeSet;
use std::ops::Bound::{Included, Unbounded};

struct Room {
    id: usize,
    size: i32,
}

struct Query {
    preferred_id: usize,
    min_size: i32,
    q_id: usize,
}

impl Solution {
    pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rooms: Vec<_> = rooms
            .iter()
            .map(|x| Room {
                id: x[0] as usize,
                size: x[1],
            })
            .collect();
        let mut queries: Vec<_> = (0..queries.len())
            .map(|i| Query {
                preferred_id: queries[i][0] as usize,
                min_size: queries[i][1],
                q_id: i,
            })
            .collect();

        rooms.sort_by_key(|r| -r.size);
        queries.sort_by_key(|q| -q.min_size);

        let mut set = BTreeSet::new();
        let mut ans = vec![-1; queries.len()];
        let mut ptr = 0;
        for q in &queries {
            while ptr < rooms.len() && rooms[ptr].size >= q.min_size {
                set.insert(rooms[ptr].id);
                ptr += 1;
            }

            let mut candidates = Vec::new();

            let range = (Unbounded, Included(q.preferred_id));
            let mut it = set.range(range);
            if let Some(&id) = it.next_back() {
                candidates.push(id);
            }

            let range = (Included(q.preferred_id), Unbounded);
            let mut it = set.range(range);
            if let Some(&id) = it.next() {
                candidates.push(id);
            }

            let mut best_id = -1;
            for &id in &candidates {
                if best_id == -1 {
                    best_id = id as i32;
                    continue;
                }
                let cur_dist = (q.preferred_id as i32 - id as i32).abs();
                let best_dist = (q.preferred_id as i32 - best_id).abs();
                if cur_dist < best_dist || (cur_dist == best_dist && (id as i32) < best_id) {
                    best_id = id as i32;
                }
            }
            ans[q.q_id] = best_id as i32;
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
    #[case(vec![vec![2,2],vec![1,2],vec![3,2]], vec![vec![3,1],vec![3,3],vec![5,2]], vec![3,-1,3])]
    #[case(vec![vec![1,4],vec![2,3],vec![3,5],vec![4,1],vec![5,2]], vec![vec![2,3],vec![2,4],vec![2,5]], vec![2,1,3])]
    fn case(
        #[case] rooms: Vec<Vec<i32>>,
        #[case] queries: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::closest_room(rooms, queries);
        assert_eq!(actual, expected);
    }
}
