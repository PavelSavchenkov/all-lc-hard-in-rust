//! Solution for https://leetcode.com/problems/cat-and-mouse
//! 913. Cat and Mouse

use std::collections::VecDeque;

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();

        let N = n * n * 2;
        let mut gt = vec![Vec::new(); N];
        let mut deg_out = vec![0; N];
        for a in 0..n {
            for b in 1..n {
                if a == 0 || a == b {
                    continue;
                }
                // mouse's turn
                {
                    let v = a * n + b;
                    for &na in &graph[a] {
                        let na = na as usize;
                        let u = n * n + (na * n + b);
                        gt[u].push(v);
                        deg_out[v] += 1;
                    }
                }
                // cat's turn
                {
                    let v = n * n + (a * n + b);
                    for &nb in &graph[b] {
                        let nb = nb as usize;
                        if nb == 0 {
                            continue;
                        }
                        let u = a * n + nb;
                        gt[u].push(v);
                        deg_out[v] += 1;
                    }
                }
            }
        }

        let is_mouse_turn = |v: usize| -> bool { v < n * n };

        let mut val = vec![0; N];
        let mut q = VecDeque::new();
        let mut cnt_out = vec![vec![0; 3]; N];
        let update = |v: usize,
                      q: &mut VecDeque<(usize, usize)>,
                      cnt_out: &mut Vec<Vec<usize>>,
                      val: &mut Vec<usize>,
                      new_val: usize| {
            if val[v] == new_val {
                return;
            }
            // eprintln!("v={}, val[v]={}, new_val={}", v, val[v], new_val);
            assert!(val[v] == 0);
            assert!(new_val == 1 || new_val == 2);
            val[v] = new_val;
            for &from in &gt[v] {
                cnt_out[from][val[v]] += 1;
                if is_mouse_turn(from) {
                    if val[v] == 1 {
                        q.push_back((from, 1));
                    }
                    if deg_out[from] == cnt_out[from][2] {
                        assert!(val[v] == 2);
                        q.push_back((from, 2));
                    }
                } else {
                    if val[v] == 2 {
                        q.push_back((from, 2));
                    }
                    if deg_out[from] == cnt_out[from][1] {
                        assert!(val[v] == 1);
                        q.push_back((from, 1));
                    }
                }
            }
        };
        for a in 0..n {
            for b in 1..n {
                let v = a * n + b;
                if a == b {
                    update(v, &mut q, &mut cnt_out, &mut val, 2);
                    update(n * n + v, &mut q, &mut cnt_out, &mut val, 2);
                }
                if a == 0 {
                    update(v, &mut q, &mut cnt_out, &mut val, 1);
                    update(n * n + v, &mut q, &mut cnt_out, &mut val, 1);
                }
            }
        }
        while !q.is_empty() {
            let (v, new_val) = q.pop_front().unwrap();
            update(v, &mut q, &mut cnt_out, &mut val, new_val);
        }

        let start_v = 1 * n + 2;
        val[start_v] as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![2,5],vec![3],vec![0,4,5],vec![1,4,5],vec![2,3],vec![0,2,3]], 0)]
    #[case(vec![vec![1,3],vec![0],vec![3],vec![0,2]], 1)]
    fn case(#[case] graph: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::cat_mouse_game(graph);
        assert_eq!(actual, expected);
    }
}
