//! Solution for https://leetcode.com/problems/shortest-path-to-get-all-keys
//! 864. Shortest Path to Get All Keys

use std::collections::VecDeque;

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn shortest_path_all_keys(g: Vec<String>) -> i32 {
        let g = to_u8_vec(&g);
        let n = g.len();
        let m = g[0].len();

        let try_go = |i: usize, j: usize, d: usize| -> Option<(usize, usize)> {
            let ni = i as i32 + di[d];
            let nj = j as i32 + dj[d];
            if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                return None;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if g[ni][nj] == b'#' {
                return None;
            }
            Some((ni, nj))
        };

        let mut k = 0;
        for i in 0..n {
            for j in 0..m {
                let ch = g[i][j];
                if ch.is_ascii_lowercase() {
                    k = k.max((ch - b'a' + 1) as usize);
                }
            }
        }

        let mut dp = vec![vec![vec![usize::MAX; 1 << k]; m]; n];
        let mut q = VecDeque::new();
        for i in 0..n {
            for j in 0..m {
                if g[i][j] == b'@' {
                    dp[i][j][0] = 0;
                    q.push_back((i, j, 0));
                }
            }
        }
        while !q.is_empty() {
            let (i, j, mask) = q.pop_front().unwrap();
            let cur_dp = dp[i][j][mask];
            if mask == (1 << k) - 1 {
                return cur_dp as i32;
            }
            for d in 0..4 {
                if let Some((ni, nj)) = try_go(i, j, d) {
                    let mut nmask = mask;
                    let ch = g[ni][nj];
                    if ch.is_ascii_lowercase() {
                        nmask |= 1 << (ch - b'a');
                    }
                    if ch.is_ascii_uppercase() && ((nmask >> (ch - b'A')) & 1) == 0 {
                        continue;
                    }
                    if dp[ni][nj][nmask] > cur_dp + 1 {
                        dp[ni][nj][nmask] = cur_dp + 1;
                        q.push_back((ni, nj, nmask));
                    }
                }
            }
        }
        -1
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
    #[case(vec!["@.a..".into(),"###.#".into(),"b.A.B".into()], 8)]
    #[case(vec!["@..aA".into(),"..B#.".into(),"....b".into()], 6)]
    #[case(vec!["@Aa".into()], -1)]
    fn case(#[case] grid: Vec<String>, #[case] expected: i32) {
        let actual = Solution::shortest_path_all_keys(grid);
        assert_eq!(actual, expected);
    }
}
