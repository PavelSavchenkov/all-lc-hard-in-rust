//! Solution for https://leetcode.com/problems/escape-a-large-maze
//! 1036. Escape a Large Maze

use std::collections::{HashSet, VecDeque};

const M: i32 = 1_000_000;

const dx: [i32; 4] = [0, 0, 1, -1];
const dy: [i32; 4] = [1, -1, 0, 0];

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(v: &Vec<i32>) -> Self {
        Self { x: v[0], y: v[1] }
    }
}

fn explore(
    src: Point,
    dst: Point,
    is_blocked: &HashSet<Point>,
    max_visited: usize,
) -> (usize, bool) {
    let mut was = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(src);
    was.insert(src);
    while !q.is_empty() && was.len() <= max_visited {
        let p = q.pop_front().unwrap();
        if p == dst {
            return (was.len(), true);
        }
        for d in 0..4 {
            let nx = p.x + dx[d];
            let ny = p.y + dy[d];
            let np = Point { x: nx, y: ny };
            if !(0 <= nx && nx < M && 0 <= ny && ny < M) {
                continue;
            }
            if is_blocked.contains(&np) {
                continue;
            }
            if was.insert(np) {
                q.push_back(np);
            }
        }
    }
    (was.len(), false)
}

impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let blocked: Vec<_> = blocked.iter().map(|v| Point::new(&v)).collect();
        let source = Point::new(&source);
        let target = Point::new(&target);

        let max_area = blocked.len().pow(2) / 2;

        let mut is_blocked = HashSet::new();
        for &p in &blocked {
            is_blocked.insert(p);
        }

        let (cnt_visited, reached) = explore(source, target, &is_blocked, max_area);
        if reached {
            return true;
        }
        if cnt_visited <= max_area {
            return false;
        }
        let (cnt_visited, reached) = explore(target, source, &is_blocked, max_area);
        if reached {
            return true;
        }
        if cnt_visited > max_area {
            return true;
        }
        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1],vec![1,0]], vec![0,0], vec![0,2], false)]
    #[case(vec![], vec![0,0], vec![999999,999999], true)]
    fn case(
        #[case] blocked: Vec<Vec<i32>>,
        #[case] source: Vec<i32>,
        #[case] target: Vec<i32>,
        #[case] expected: bool,
    ) {
        let actual = Solution::is_escape_possible(blocked, source, target);
        assert_eq!(actual, expected);
    }
}
