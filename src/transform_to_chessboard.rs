//! Solution for https://leetcode.com/problems/transform-to-chessboard
//! 782. Transform to Chessboard

use std::collections::HashMap;

fn solve_rows(b: &Vec<Vec<i32>>) -> i32 {
    let n = b.len();
    let mut cnt = HashMap::new();
    let mut masks = Vec::new();
    for i in 0..n {
        let mut mask = 0;
        for j in (0..n).rev() {
            mask = mask * 2 + b[i][j] as usize;
        }
        masks.push(mask);
        *cnt.entry(mask).or_insert(0) += 1;
    }

    if cnt.len() != 2 {
        return -1;
    }

    let mut ms: Vec<usize> = cnt.iter().map(|(k, v)| *k).collect();
    assert!(ms.len() == 2);
    if (ms[0] & ms[1]) != 0 {
        return -1;
    }
    if ms[0].count_ones() + ms[1].count_ones() != n as u32 {
        return -1;
    }

    if n % 2 == 0 {
        if *cnt.get(&ms[0]).unwrap() != n / 2 {
            return -1;
        }
        let mut cnt = vec![0; 2];
        for i in (0..n).step_by(2) {
            for j in 0..2 {
                if masks[i] == ms[j] {
                    cnt[j] += 1;
                }
            }
        }
        return cnt[0].min(cnt[1]);
    }

    if *cnt.get(&ms[0]).unwrap() <= n / 2 {
        ms.swap(0, 1);
    }

    let mut ans = 0;
    for i in (0..n).step_by(2) {
        if masks[i] != ms[0] {
            ans += 1;
        }
    }
    ans
}

impl Solution {
    pub fn moves_to_chessboard(b: Vec<Vec<i32>>) -> i32 {
        let n = b.len();
        let rows = solve_rows(&b);
        let mut b_transposed = b.clone();
        for i in 0..n {
            for j in 0..n {
                b_transposed[i][j] = b[j][i];
            }
        }
        let cols = solve_rows(&b_transposed);

        if rows == -1 || cols == -1 {
            return -1;
        }
        rows + cols
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1,1,0],vec![0,1,1,0],vec![1,0,0,1],vec![1,0,0,1]], 2)]
    #[case(vec![vec![0,1],vec![1,0]], 0)]
    #[case(vec![vec![1,0],vec![1,0]], -1)]
    fn case(#[case] board: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::moves_to_chessboard(board);
        assert_eq!(actual, expected);
    }
}
