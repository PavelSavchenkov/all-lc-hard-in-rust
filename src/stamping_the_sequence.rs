//! Solution for https://leetcode.com/problems/stamping-the-sequence
//! 936. Stamping The Sequence

use std::collections::VecDeque;

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp = to_u8(&stamp);
        let mut target = to_u8(&target);
        let n = target.len();
        let k = stamp.len();

        let update = |row: usize,
                      col: usize,
                      g: &mut Vec<Vec<bool>>,
                      cnt_ones_in_row: &mut Vec<usize>,
                      q: &mut VecDeque<usize>| {
            assert!(row <= n - k);
            if g[row][col] {
                return;
            }
            // eprintln!("update row = {}, col = {}", row, col);
            g[row][col] = true;
            cnt_ones_in_row[row] += 1;
            if cnt_ones_in_row[row] == k {
                q.push_back(row);
            }
        };

        let mut g = vec![vec![false; n]; n - k + 1];
        let mut cnt_ones_in_row = vec![0; n - k + 1];
        let mut q = VecDeque::new();
        for i in 0..=n - k {
            for j in 0..k {
                if target[i + j] == stamp[j] {
                    update(i, j, &mut g, &mut cnt_ones_in_row, &mut q);
                }
            }
        }
        let mut ans = Vec::new();
        while !q.is_empty() {
            let i = q.pop_front().unwrap();
            // eprintln!("process i = {}", i);
            assert!(cnt_ones_in_row[i] == k);
            ans.push(i as i32);
            for j in 0..k {
                if target[i + j] == b'?' {
                    continue;
                }
                target[i + j] = b'?';
                let left = if i + j + 1 >= k { i + j + 1 - k } else { 0 };
                for ii in left..=(i + j).min(n - k) {
                    update(ii, (i + j) - ii, &mut g, &mut cnt_ones_in_row, &mut q);
                }
            }
        }
        for i in 0..n {
            if target[i] != b'?' {
                return Vec::new();
            }
        }
        ans.reverse();
        ans
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
    #[case("abc", "ababc", vec![0,2])]
    // #[case("abca", "aabcaca", vec![3,0,1])]
    fn case(#[case] stamp: String, #[case] target: String, #[case] expected: Vec<i32>) {
        let actual = Solution::moves_to_stamp(stamp, target);
        assert_eq!(actual, expected);
    }
}
