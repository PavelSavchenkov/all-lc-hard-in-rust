//! Solution for https://leetcode.com/problems/my-calendar-iii
//! 732. My Calendar III

use std::collections::BTreeSet;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum EventType {
    Open,
    Close,
}

#[derive(Default)]
struct MyCalendarThree {
    events: BTreeSet<(i32, EventType, usize)>,
    book_id: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        Self::default()
    }

    fn book(&mut self, l: i32, r: i32) -> i32 {
        self.book_id += 1;
        self.events.insert((l, EventType::Open, self.book_id));
        self.events.insert((r, EventType::Close, self.book_id));

        let mut mx = 0;
        let mut bal = 0;
        let mut prev_x = i32::MIN;
        for &(x, t, _) in &self.events {
            assert!(prev_x <= x);
            if x != prev_x {
                mx = mx.max(bal);
                assert!(bal >= 0);
            }
            bal += if t == EventType::Open { 1 } else { -1 };
            prev_x = x;
        }
        mx = mx.max(bal);
        mx
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(startTime, endTime);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
