//! Solution for https://leetcode.com/problems/robot-collisions
//! 2751. Robot Collisions

use std::collections::VecDeque;

#[derive(Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

struct Robot {
    position: i32,
    health: i32,
    direction: Direction,
    i: usize,
}

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let n = positions.len();
        let directions = to_u8(&directions);

        let mut robots = Vec::new();
        for i in 0..n {
            robots.push(Robot {
                position: positions[i],
                health: healths[i],
                direction: if directions[i] == b'L' {
                    Direction::Left
                } else {
                    Direction::Right
                },
                i,
            });
        }
        robots.sort_by_key(|r| r.position);
        drop(positions);
        drop(healths);
        drop(directions);

        let remove = |i: usize,
                      next: &mut Vec<Option<usize>>,
                      prev: &mut Vec<Option<usize>>,
                      head: &mut Option<usize>| {
            assert!(head.is_some());
            let left = prev[i];
            let right = next[i];
            if i == head.unwrap() {
                *head = right;
                if right.is_some() {
                    prev[right.unwrap()] = None;
                }
            } else {
                if left.is_some() {
                    next[left.unwrap()] = right;
                }
                if right.is_some() {
                    prev[right.unwrap()] = left;
                }
            }
            prev[i] = None;
            next[i] = None;
        };

        let mut prev = vec![None; n];
        let mut next = vec![None; n];
        for i in 0..n {
            if i > 0 {
                prev[i] = Some(i - 1);
            }
            if i + 1 < n {
                next[i] = Some(i + 1);
            }
        }
        let mut head = Some(0 as usize);

        let mut q = VecDeque::new();
        for i in 0..n - 1 {
            if robots[i].direction == Direction::Right && robots[i + 1].direction == Direction::Left
            {
                q.push_back(i);
            }
        }
        while !q.is_empty() {
            let i = q.pop_front().unwrap();
            let next_i = next[i];
            if next_i.is_none() {
                continue;
            }
            let next_i = next_i.unwrap();
            if !(robots[i].direction == Direction::Right
                && robots[next_i].direction == Direction::Left)
            {
                continue;
            }
            let mut candidates = Vec::new();
            candidates.push(prev[i]);
            candidates.push(next[next_i]);
            candidates.push(Some(i));
            candidates.push(Some(next_i));
            eprintln!("collide {} {}", i, next_i);
            if robots[i].health < robots[next_i].health {
                remove(i, &mut next, &mut prev, &mut head);
                robots[next_i].health -= 1;
                candidates.push(Some(next_i));
            } else if robots[i].health > robots[next_i].health {
                remove(next_i, &mut next, &mut prev, &mut head);
                robots[i].health -= 1;
                candidates.push(Some(i));
            } else {
                remove(i, &mut next, &mut prev, &mut head);
                remove(next_i, &mut next, &mut prev, &mut head);
            }
            for j in &candidates {
                if j.is_some() {
                    q.push_front(j.unwrap());
                }
            }
        }

        let mut ans = Vec::new();
        while head.is_some() {
            ans.push(head.unwrap());
            head = next[head.unwrap()];
        }
        ans.sort_by_key(|&i| robots[i].i);

        let mut final_healths = Vec::new();
        for &i in &ans {
            final_healths.push(robots[i].health);
        }

        final_healths
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![5,4,3,2,1], vec![2,17,9,15,10], "RRRRR", vec![2,17,9,15,10])]
    #[case(vec![3,5,2,6], vec![10,10,15,12], "RLRL", vec![14])]
    #[case(vec![1,2,5,6], vec![10,10,11,11], "RLRL", vec![])]
    fn case(
        #[case] positions: Vec<i32>,
        #[case] healths: Vec<i32>,
        #[case] directions: String,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::survived_robots_healths(positions, healths, directions);
        assert_eq!(actual, expected);
    }
}
