//! Solution for https://leetcode.com/problems/sliding-puzzle
//! 773. Sliding Puzzle

use std::collections::HashSet;

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn sliding_puzzle(g_zero: Vec<Vec<i32>>) -> i32 {
        let n = g_zero.len();
        let m = g_zero[0].len();

        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((g_zero.clone(), 0));
        visited.insert(get_hash(&g_zero));
        while !q.is_empty() {
            let (g, dist) = q.pop_front().unwrap();

            let mut is_final = true;
            for i in 0..n {
                for j in 0..m {
                    if g[i][j] as usize != (1 + i * m + j) % (n * m) {
                        is_final = false;
                    }
                }
            }
            if is_final {
                return dist;
            }

            for i in 0..n {
                for j in 0..m {
                    if g[i][j] != 0 {
                        continue;
                    }
                    for d in 0..4 {
                        let ni = i as i32 + di[d];
                        let nj = j as i32 + dj[d];
                        if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                            continue;
                        }
                        let ni = ni as usize;
                        let nj = nj as usize;
                        let mut ng = g.clone();
                        ng[i][j] = ng[ni][nj];
                        ng[ni][nj] = 0;
                        let h = get_hash(&ng);
                        if visited.contains(&h) {
                            continue;
                        }
                        visited.insert(h);
                        q.push_back((ng, dist + 1));
                    }
                }
            }
        }
        -1
    }
}

use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2,3],vec![4,0,5]], 1)]
    #[case(vec![vec![1,2,3],vec![5,4,0]], -1)]
    #[case(vec![vec![4,1,2],vec![5,0,3]], 5)]
    fn case(#[case] board: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::sliding_puzzle(board);
        assert_eq!(actual, expected);
    }
}
