//! Solution for https://leetcode.com/problems/sequentially-ordinal-rank-tracker
//! 2102. Sequentially Ordinal Rank Tracker

use std::ops::Bound::*;
use std::{collections::BTreeSet, default};

#[derive(PartialEq, Eq, Clone, Debug)]
struct Location {
    score: i32,
    name: String,
}

impl Location {
    fn new(name: &String, score: i32) -> Self {
        Self {
            name: name.to_string(),
            score,
        }
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score != other.score {
            return self.score.cmp(&other.score);
        }
        other.name.cmp(&self.name)
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct SORTracker {
    set: BTreeSet<Location>,
    location_ptr: Option<Location>,
    cnt_bigger_than_location_ptr: usize,
    get_cnt: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SORTracker {
    fn new() -> Self {
        Self {
            set: Default::default(),
            location_ptr: None,
            cnt_bigger_than_location_ptr: 0,
            get_cnt: 0,
        }
    }

    fn add(&mut self, name: String, score: i32) {
        let location = Location::new(&name, score);
        // eprintln!("add {:#?}", location);
        if !self.location_ptr.is_none() && &location > self.location_ptr.as_ref().unwrap() {
            self.cnt_bigger_than_location_ptr += 1;
        }
        self.set.insert(location.clone());
        if self.location_ptr.is_none() {
            self.location_ptr = Some(location);
            assert!(self.cnt_bigger_than_location_ptr == 0);
            assert!(self.set.len() == 1);
        }
    }

    fn get(&mut self) -> String {
        // eprintln!("get self.get_cnt = {}", self.get_cnt);
        // eprintln!(
        //     "cnt_bigger than: {}, location_ptr: {:#?}",
        //     self.cnt_bigger_than_location_ptr, self.location_ptr
        // );
        self.move_ptr_until(self.get_cnt);
        // eprintln!(
        //     "after move\ncnt_bigger than: {}, location_ptr: {:#?}",
        //     self.cnt_bigger_than_location_ptr, self.location_ptr
        // );
        self.get_cnt += 1;
        self.location_ptr.as_ref().unwrap().name.clone()
    }

    fn get_prev(&self, location: &Location) -> Option<Location> {
        // biggest elem less than
        let it = self.set.range(..location);
        it.last().cloned()
    }

    fn get_next(&self, location: &Location) -> Option<Location> {
        // smallest elem greater than
        let it = self.set.range((Excluded(location), Unbounded));
        it.min().cloned()
    }

    fn move_ptr_until(&mut self, need_cnt: usize) {
        while self.cnt_bigger_than_location_ptr != need_cnt {
            if self.cnt_bigger_than_location_ptr > need_cnt {
                self.location_ptr = self.get_next(self.location_ptr.as_ref().unwrap());
                self.cnt_bigger_than_location_ptr -= 1;
            } else {
                self.location_ptr = self.get_prev(self.location_ptr.as_ref().unwrap());
                self.cnt_bigger_than_location_ptr += 1;
            }
        }
    }
}

/**
 * Your SORTracker object will be instantiated and called as such:
 * let obj = SORTracker::new();
 * obj.add(name, score);
 * let ret_2: String = obj.get();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case()]
    fn case() {
        let queries: Vec<(&str, i32)> = vec![
            ("bradford", 2),
            ("branford", 3),
            ("branford", -1),
            ("alps", 2),
            ("alps", -1),
            ("orland", 2),
            ("bradford", -1),
            ("orland", 3),
            ("bradford", -1),
            ("alpine", 2),
            ("bradford", -1),
            ("orland", -1),
        ];

        let mut obj = SORTracker::new();
        for (name, score) in queries {
            if score == -1 {
                let res = obj.get();
                assert_eq!(res, name.to_string());
            } else {
                obj.add(name.to_string(), score);
            }
        }
    }
}
