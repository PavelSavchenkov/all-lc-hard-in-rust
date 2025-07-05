//! Solution for https://leetcode.com/problems/number-of-valid-move-combinations-on-chessboard
//! 2056. Number of Valid Move Combinations On Chessboard

enum Piece {
    Rook,
    Queen,
    Bishop,
}

#[derive(PartialEq, Clone, Debug)]
struct Pos {
    row: usize,
    col: usize,
}

fn check(pieces: &Vec<Piece>, init: &Vec<Pos>, target: &mut Vec<Pos>) -> bool {
    let mut pos = init.clone();
    let mut was = vec![vec![false; 8]; 8];
    loop {
        let mut all_on_target = true;
        for i in 0..pieces.len() {
            if pos[i] != target[i] {
                all_on_target = false
            }
            let p = &pos[i];
            if was[p.row][p.col] {
                return false;
            }
            was[p.row][p.col] = true;
        }
        if all_on_target {
            return true;
        }
        for i in 0..pieces.len() {
            let p = &mut pos[i];
            was[p.row][p.col] = false;
            let dr = (target[i].row as i32 - p.row as i32).signum();
            let dc = (target[i].col as i32 - p.col as i32).signum();
            if dr == 1 {
                p.row += 1;
            } else if dr == -1 {
                p.row -= 1;
            }
            if dc == 1 {
                p.col += 1;
            } else if dc == -1 {
                p.col -= 1;
            }
        }
    }
}

fn go(pieces: &Vec<Piece>, init: &Vec<Pos>, target: &mut Vec<Pos>, ans: &mut usize) {
    if target.len() == init.len() {
        if check(pieces, init, target) {
            *ans += 1;
        }
        return;
    }

    let i = target.len();
    for r in 0..8 {
        for c in 0..8 {
            let ok_straight = r == init[i].row || c == init[i].col;
            let ok_diag = r + c == init[i].row + init[i].col
                || r as i32 - c as i32 == init[i].row as i32 - init[i].col as i32;
            let can_reach = match pieces[i] {
                Piece::Bishop => ok_diag,
                Piece::Queen => ok_straight || ok_diag,
                Piece::Rook => ok_straight,
            };
            if !can_reach {
                continue;
            }
            target.push(Pos { row: r, col: c });
            go(pieces, init, target, ans);
            target.pop();
        }
    }
}

impl Solution {
    pub fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32 {
        let pieces: Vec<_> = pieces
            .iter()
            .map(|p| match p.as_str() {
                "rook" => Piece::Rook,
                "queen" => Piece::Queen,
                "bishop" => Piece::Bishop,
                _ => {
                    panic!("Didn't recognize the piece {}", p)
                }
            })
            .collect();

        let init: Vec<_> = positions
            .iter()
            .map(|p| Pos {
                row: (p[0] - 1) as usize,
                col: (p[1] - 1) as usize,
            })
            .collect();

        let mut target = Vec::<Pos>::new();
        let mut ans = 0;
        go(&pieces, &init, &mut target, &mut ans);

        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["rook".into()], vec![vec![1,1]], 15)]
    #[case(vec!["queen".into()], vec![vec![1,1]], 22)]
    #[case(vec!["bishop".into()], vec![vec![4,3]], 12)]
    #[case(vec!["rook".into(), "rook".into()], vec![vec![1,1], vec![8,8]], 223)]
    fn case(#[case] pieces: Vec<String>, #[case] positions: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::count_combinations(pieces, positions);
        assert_eq!(actual, expected);
    }
}
