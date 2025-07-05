//! Solution for https://leetcode.com/problems/time-to-cross-a-bridge
//! 2532. Time to Cross a Bridge

use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Eq, PartialEq, Debug)]
struct Worker {
    right: u32,
    pick: u32,
    left: u32,
    put: u32,
}

impl Worker {
    fn new(v: &Vec<i32>) -> Self {
        Self {
            right: v[0] as u32,
            pick: v[1] as u32,
            left: v[2] as u32,
            put: v[3] as u32,
        }
    }

    fn bridge_time(&self) -> u32 {
        self.left + self.right
    }
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bridge_time().cmp(&other.bridge_time())
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Debug)]
enum EventType {
    ReachedRight,
    Picked,
    ReachedLeft,
    Put,
}

#[derive(Eq, PartialEq, Debug)]
struct Event {
    t: u32,
    i: usize,
    type_: EventType,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.t, self.i).cmp(&(other.t, other.i))
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
        let mut n = n as usize;
        let k = k as usize;

        let mut workers = Vec::with_capacity(k);
        for i in 0..k {
            workers.push(Worker::new(&time[i]));
        }

        // stable sort
        workers.sort();
        workers.reverse();

        let mut left = BTreeSet::<usize>::new();
        for i in 0..k {
            left.insert(i);
        }
        let mut right = BTreeSet::<usize>::new();
        let mut is_bridge_free = true;
        let mut es = BTreeSet::<Event>::new();
        let mut last_t = 0;
        let mut last_reach_left = 0;
        while n > 0 || !es.is_empty() || !right.is_empty() {
            if is_bridge_free {
                if !right.is_empty() {
                    let i = right.pop_first().unwrap();
                    es.insert(Event {
                        t: last_t + workers[i].left,
                        i,
                        type_: EventType::ReachedLeft,
                    });
                    is_bridge_free = false;
                    continue;
                } else if n > 0 && !left.is_empty() {
                    let i = left.pop_first().unwrap();
                    es.insert(Event {
                        t: last_t + workers[i].right,
                        i,
                        type_: EventType::ReachedRight,
                    });
                    is_bridge_free = false;
                    assert!(n > 0);
                    n -= 1;
                    continue;
                }
            }

            assert!(!es.is_empty());
            let t = es.first().unwrap().t;
            last_t = t;
            while !es.is_empty() && es.first().unwrap().t == t {
                let e = es.pop_first().unwrap();
                let i = e.i;
                match e.type_ {
                    EventType::ReachedRight => {
                        es.insert(Event {
                            t: t + workers[i].pick,
                            i,
                            type_: EventType::Picked,
                        });
                        is_bridge_free = true;
                    }
                    EventType::Picked => {
                        right.insert(i);
                    }
                    EventType::ReachedLeft => {
                        es.insert(Event {
                            t: t + workers[i].put,
                            i,
                            type_: EventType::Put,
                        });
                        is_bridge_free = true;
                        last_reach_left = t;
                    }
                    EventType::Put => {
                        left.insert(i);
                    }
                }
            }
        }
        assert!(right.is_empty());

        last_reach_left as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 3, vec![vec![1,1,2,1],vec![1,1,3,1],vec![1,1,4,1]], 6)]
    // #[case(3, 2, vec![vec![1,5,1,8],vec![10,10,10,10]], 37)]
    fn case(#[case] n: i32, #[case] k: i32, #[case] time: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::find_crossing_time(n, k, time);
        assert_eq!(actual, expected);
    }
}
