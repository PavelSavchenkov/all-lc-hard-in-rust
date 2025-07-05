//! Solution for https://leetcode.com/problems/the-skyline-problem
//! 218. The Skyline Problem

use std::collections::BTreeSet;

#[derive(PartialEq)]
enum EventType {
    Start,
    End,
}

struct Event {
    x: u32,
    h: u32,
    id: usize,
    t: EventType,
}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut es = Vec::new();
        for (id, b) in buildings.iter().enumerate() {
            let h = b[2] as u32;
            es.push(Event {
                x: b[0] as u32,
                h,
                id,
                t: EventType::Start,
            });
            es.push(Event {
                x: b[1] as u32,
                h,
                id,
                t: EventType::End,
            });
        }

        es.sort_by_key(|e| e.x);

        let mut ans = Vec::<Vec<i32>>::new();
        let mut set = BTreeSet::new();
        let mut i = 0;
        while i < es.len() {
            let mut j = i;
            while j < es.len() && es[j].x == es[i].x {
                j += 1;
            }

            for k in i..j {
                let elem = (es[k].h, es[k].id);
                if es[k].t == EventType::Start {
                    set.insert(elem);
                } else {
                    set.remove(&elem);
                }
            }

            let h = set.last().unwrap_or(&(0, 0)).0;
            let pt = vec![es[i].x as i32, h as i32];
            if ans.is_empty() || ans.last().unwrap()[1] != pt[1] {
                ans.push(pt);
            }

            i = j;
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
    #[case(vec![vec![2,9,10],vec![3,7,15],vec![5,12,12],vec![15,20,10],vec![19,24,8]], vec![vec![2,10],vec![3,15],vec![7,12],vec![12,0],vec![15,10],vec![20,8],vec![24,0]])]
    #[case(vec![vec![0,2,3],vec![2,5,3]], vec![vec![0,3],vec![5,0]])]
    fn case(#[case] buildings: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::get_skyline(buildings);
        assert_eq!(actual, expected);
    }
}
