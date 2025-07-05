//! Solution for https://leetcode.com/problems/perfect-rectangle
//! 391. Perfect Rectangle

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

#[derive(PartialEq)]
enum EventType {
    Add,
    Del,
}

struct Event {
    x: i32,
    y0: i32,
    y1: i32,
    t: EventType,
}

impl Event {
    fn get_seg(&self) -> (i32, i32) {
        (self.y0, self.y1)
    }
}

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let mut es = Vec::new();
        let mut total_area: i64 = 0;
        for r in &rectangles {
            let x0 = r[0];
            let y0 = r[1];
            let x1 = r[2];
            let y1 = r[3];

            es.push(Event {
                x: x0,
                y0,
                y1: y1 - 1,
                t: EventType::Add,
            });
            es.push(Event {
                x: x1 - 1,
                y0,
                y1: y1 - 1,
                t: EventType::Del,
            });

            total_area += (x1 - x0) as i64 * (y1 - y0) as i64;
            min_x = min_x.min(x0);
            min_y = min_y.min(y0);
            max_x = max_x.max(x1);
            max_y = max_y.max(y1);
        }

        if total_area != (max_x - min_x) as i64 * (max_y - min_y) as i64 {
            eprintln!(
                "max_x = {}, max_y = {}, min_x = {}, min_y = {}",
                max_x, max_y, min_x, min_y
            );
            eprintln!("Areas don't match");
            return false;
        }

        es.sort_by_key(|e| e.x);

        let mut segs = BTreeSet::new();
        let mut i = 0;
        while i < es.len() {
            let mut j = i;
            while j < es.len() && es[i].x == es[j].x {
                j += 1;
            }

            for k in i..j {
                if es[k].t == EventType::Add {
                    let (l, r) = es[k].get_seg();
                    let range = (Unbounded, Excluded((r, i32::MAX)));
                    let it = segs.range(range);
                    if let Some((L, R)) = it.copied().next_back() {
                        if R >= l {
                            eprintln!("Intersection at x = {}", es[i].x);
                            return false;
                        }
                    }
                    segs.insert((l, r));
                }
            }

            for k in i..j {
                if es[k].t == EventType::Del {
                    segs.remove(&es[k].get_seg());
                }
            }

            i = j;
        }

        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,1,3,3],vec![3,1,4,2],vec![3,2,4,4],vec![1,3,2,4],vec![2,3,3,4]], true)]
    #[case(vec![vec![1,1,2,3],vec![1,3,2,4],vec![3,1,4,2],vec![3,2,4,4]], false)]
    #[case(vec![vec![1,1,3,3],vec![3,1,4,2],vec![1,3,2,4],vec![2,2,4,4]], false)]
    fn case(#[case] rectangles: Vec<Vec<i32>>, #[case] expected: bool) {
        let actual = Solution::is_rectangle_cover(rectangles);
        assert_eq!(actual, expected);
    }
}
