//! Solution for https://leetcode.com/problems/meeting-rooms-iii
//! 2402. Meeting Rooms III

struct Meeting {
    start: u32,
    end: u32,
}

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut ms = Vec::new();
        for m in &meetings {
            ms.push(Meeting {
                start: m[0] as u32,
                end: m[1] as u32,
            });
        }
        ms.sort_by_key(|m| m.start);
        let meetings = ms;

        let mut cnt_used = vec![0; n];
        let mut when_open = vec![0 as u64; n];
        for m in &meetings {
            let mut best_room = 0;
            let mut best_when = u64::MAX;
            for room in 0..n {
                let when = (m.start as u64).max(when_open[room]);
                if when < best_when {
                    best_when = when;
                    best_room = room;
                }
            }
            when_open[best_room] = best_when + (m.end - m.start) as u64;
            cnt_used[best_room] += 1;
        }

        let mut mx_room = 0;
        for room in 0..n {
            if cnt_used[room] > cnt_used[mx_room] {
                mx_room = room;
            }
        }
        mx_room as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, vec![vec![0,10],vec![1,5],vec![2,7],vec![3,4]], 0)]
    #[case(3, vec![vec![1,20],vec![2,10],vec![3,5],vec![4,9],vec![6,8]], 1)]
    fn case(#[case] n: i32, #[case] meetings: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::most_booked(n, meetings);
        assert_eq!(actual, expected);
    }
}
