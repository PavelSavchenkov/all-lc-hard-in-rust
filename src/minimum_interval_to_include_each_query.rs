//! Solution for https://leetcode.com/problems/minimum-interval-to-include-each-query
//! 1851. Minimum Interval to Include Each Query

use std::collections::BTreeSet;

struct Interval {
    left: i32,
    right: i32,
}

impl Interval {
    fn len(&self) -> i32 {
        self.right - self.left + 1
    }
}

struct Query {
    x: i32,
    id: usize,
}

enum EventType {
    Open,
    Close,
}

struct Event {
    x: i32,
    id: usize,
    event_type: EventType,
}

impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let intervals: Vec<_> = intervals
            .iter()
            .map(|x| Interval {
                left: x[0],
                right: x[1],
            })
            .collect();
        let mut queries: Vec<_> = (0..queries.len())
            .map(|i| Query {
                x: queries[i],
                id: i,
            })
            .collect();

        let mut es = Vec::new();
        for i in 0..intervals.len() {
            es.push(Event {
                x: intervals[i].left,
                id: i,
                event_type: EventType::Open,
            });
            es.push(Event {
                x: intervals[i].right + 1,
                id: i,
                event_type: EventType::Close,
            });
        }

        es.sort_by_key(|e| e.x);

        let mut ans = vec![-1; queries.len()];
        let mut ptr = 0;
        queries.sort_by_key(|q| q.x);
        let mut set = BTreeSet::new();
        for q in &queries {
            while ptr < es.len() && es[ptr].x <= q.x {
                let e = &es[ptr];
                ptr += 1;
                let set_item = (intervals[e.id].len(), e.id);
                match e.event_type {
                    EventType::Open => {
                        set.insert(set_item);
                    }
                    EventType::Close => {
                        set.remove(&set_item);
                    }
                }
            }
            if !set.is_empty() {
                ans[q.id] = set.first().unwrap().0;
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
    #[case(vec![vec![1,4],vec![2,4],vec![3,6],vec![4,4]], vec![2,3,4,5], vec![3,3,1,4])]
    #[case(vec![vec![2,3],vec![2,5],vec![1,8],vec![20,25]], vec![2,19,5,22], vec![2,-1,4,6])]
    fn case(
        #[case] intervals: Vec<Vec<i32>>,
        #[case] queries: Vec<i32>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::min_interval(intervals, queries);
        assert_eq!(actual, expected);
    }
}
