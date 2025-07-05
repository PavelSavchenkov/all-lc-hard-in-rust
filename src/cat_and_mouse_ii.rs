//! Solution for https://leetcode.com/problems/cat-and-mouse-ii
//! 1728. Cat and Mouse II

// todo: proper retro-analysis

const ITERS: usize = 1000;

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn can_mouse_win(g: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        let g = to_u8_vec(&g);
        let n = g.len();
        let m = g[0].len();

        let mut food = n * m;
        let mut cat_initial = n * m;
        let mut mouse_initial = n * m;
        for i in 0..n {
            for j in 0..m {
                let pos = i * m + j;
                match g[i][j] {
                    b'C' => cat_initial = pos,
                    b'M' => mouse_initial = pos,
                    b'F' => food = pos,
                    _ => continue,
                }
            }
        }
        assert!(food != n * m);

        let get_all_valid_moves = |pos: usize, max_len: i32| -> Vec<usize> {
            let i = pos / m;
            let j = pos % m;
            let mut ans = Vec::new();
            for dir in 0..4 {
                for len in 1..=max_len {
                    let ni = i as i32 + len * di[dir];
                    let nj = j as i32 + len * dj[dir];
                    if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                        break;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if g[ni][nj] == b'#' {
                        break;
                    }
                    ans.push(ni * m + nj);
                }
            }
            ans.push(i * m + j);
            ans
        };

        let mut all_cat_moves_from = vec![Vec::new(); n * m];
        for pos in 0..n * m {
            all_cat_moves_from[pos] = get_all_valid_moves(pos, cat_jump);
        }
        let mut all_mouse_moves_from = vec![Vec::new(); n * m];
        for pos in 0..n * m {
            all_mouse_moves_from[pos] = get_all_valid_moves(pos, mouse_jump);
        }

        let check_terminal = |cat: usize, mouse: usize, is_mouse_move: bool| -> i32 {
            if mouse == food && cat != food && !is_mouse_move {
                return 1;
            }
            if cat == food && mouse != food && is_mouse_move {
                return 0;
            }
            if cat == mouse && is_mouse_move {
                return 0;
            }
            -1
        };

        assert!(cat_initial != n * m);
        assert!(mouse_initial != n * m);

        // cat_pos, mouse_pos, is_mouse_move --> -1, 0, 1; -1 - unknown, 0 - cat wins, 1 - mouse
        let mut dp = vec![-1; (n * m) * (n * m) * 2];
        for it in 0..ITERS {
            let mut ndp = vec![-1; (n * m) * (n * m) * 2];
            for cat in 0..n * m {
                for mouse in 0..n * m {
                    for is_mouse_move in 0..2 {
                        let cand = check_terminal(cat, mouse, is_mouse_move == 1);
                        let mut new_dp = -1;
                        if cand != -1 {
                            new_dp = cand;
                        } else if is_mouse_move == 1 {
                            let mut all_zero = true;
                            for &pos in &all_mouse_moves_from[mouse] {
                                let next_dp = dp[2 * (cat * n * m + pos)];
                                if next_dp == 1 {
                                    new_dp = 1;
                                }
                                if next_dp != 0 {
                                    all_zero = false;
                                }
                            }
                            if all_zero {
                                new_dp = 0;
                            }
                        } else {
                            let mut all_one = true;
                            for &pos in &all_cat_moves_from[cat] {
                                let next_dp = dp[2 * (pos * n * m + mouse) + 1];
                                if next_dp == 0 {
                                    new_dp = 0;
                                }
                                if next_dp != 1 {
                                    all_one = false;
                                }
                            }
                            if all_one {
                                new_dp = 1;
                            }
                        }
                        ndp[2 * (cat * n * m + mouse) + is_mouse_move] = new_dp;
                    }
                }
            }

            dp = ndp;
        }

        let func = dp[2 * (cat_initial * n * m + mouse_initial) + 1];
        func == 1
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
    #[case(vec!["####F".into(),"#C...".into(),"M....".into()], 1, 2, true)]
    #[case(vec!["M.C...F".into()], 1, 4, true)]
    #[case(vec!["M.C...F".into()], 1, 3, false)]
    fn case(
        #[case] grid: Vec<String>,
        #[case] cat_jump: i32,
        #[case] mouse_jump: i32,
        #[case] expected: bool,
    ) {
        let actual = Solution::can_mouse_win(grid, cat_jump, mouse_jump);
        assert_eq!(actual, expected);
    }
}
