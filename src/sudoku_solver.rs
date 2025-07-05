//! Solution for https://leetcode.com/problems/sudoku-solver
//! 37. Sudoku Solver

const ROWS: usize = 9 * 9 * 9;
const COLS: usize = 9 * 9 + 9 * 9 + 9 * 9 + 9 * 9;

fn decode_row_id(mut id: usize) -> (usize, usize, usize) {
    let d = id % 9;
    id /= 9;
    let j = id % 9;
    id /= 9;
    let i = id;
    assert!(i < 9);
    return (i, j, d);
}

#[derive(Default, Clone, Copy, Debug)]
struct Node {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

impl Node {
    fn new() -> Self {
        Self::default()
    }
}

enum Op {
    KillCol(usize),
    RemoveNode(usize, usize),
}

struct Solver {
    // main toroidal grid
    grid: Vec<[Node; COLS]>,
    // index of toroidal list head of a given column
    head_of_col: [Option<usize>; COLS],
    // toroidal list of alive columns
    alive_cols_list: [Node; COLS],
    // head of the list of alive cols (i.e. index of any alive column)
    alive_cols_list_head: Option<usize>,
    // how many alive ones in a given column
    cnt_who_cover_this_col: [usize; COLS],
    // sudoku board
    board: [[i32; 9]; 9],
    // stack of operations on grid
    ops: Vec<Op>,
    // info only for assertions vvv
    // dense binary grid
    cover: Vec<Vec<bool>>,
    // is call killed
    is_col_killed: Vec<bool>,
}

impl Solver {
    fn pop_op(&mut self) {
        assert!(!self.ops.is_empty());
        let op = self.ops.pop().unwrap();
        match op {
            Op::KillCol(col) => {
                self.unkill_col(col);
            }
            Op::RemoveNode(row, col) => {
                self.unremove_node(row, col);
            }
        }
    }

    fn kill_col(&mut self, col: usize) {
        self.ops.push(Op::KillCol(col));

        assert!(!self.is_col_killed[col]);
        self.is_col_killed[col] = true;

        let col_left = self.alive_cols_list[col].left;
        let col_right = self.alive_cols_list[col].right;
        self.alive_cols_list[col_left].right = col_right;
        self.alive_cols_list[col_right].left = col_left;
        self.alive_cols_list_head = if col != col_left {
            Some(col_left)
        } else {
            None
        };
    }

    fn unkill_col(&mut self, col: usize) {
        assert!(self.is_col_killed[col]);
        self.is_col_killed[col] = false;

        let col_left = self.alive_cols_list[col].left;
        let col_right = self.alive_cols_list[col].right;
        self.alive_cols_list[col_left].right = col;
        self.alive_cols_list[col_right].left = col;
        self.alive_cols_list_head = Some(col);
    }

    fn remove_node(&mut self, row: usize, col: usize) {
        self.ops.push(Op::RemoveNode(row, col));

        assert!(self.cover[row][col]);
        self.cover[row][col] = false;

        let g = &mut self.grid;

        self.cnt_who_cover_this_col[col] -= 1;

        let node = g[row][col];

        let left = node.left;
        let right = node.right;
        assert!(g[row][left].right == col);
        g[row][left].right = right;
        assert!(g[row][right].left == col);
        g[row][right].left = left;

        let up = node.up;
        let down = node.down;
        assert!(g[up][col].down == row);
        g[up][col].down = down;
        assert!(g[down][col].up == row);
        g[down][col].up = up;

        self.head_of_col[col] = if up != row { Some(up) } else { None };
    }

    fn unremove_node(&mut self, row: usize, col: usize) {
        assert!(!self.cover[row][col]);
        self.cover[row][col] = true;

        let g = &mut self.grid;

        self.cnt_who_cover_this_col[col] += 1;

        let node = g[row][col];

        let left = node.left;
        let right = node.right;
        g[row][left].right = col;
        g[row][right].left = col;

        let up = node.up;
        let down = node.down;
        g[up][col].down = row;
        g[down][col].up = row;

        self.head_of_col[col] = Some(row);
    }

    fn remove_row(&mut self, row: usize, col: usize) {
        let j0 = col;
        let mut j = self.grid[row][j0].right;
        loop {
            self.remove_node(row, j);
            if j == j0 {
                break;
            }
            j = self.grid[row][j].right;
        }
    }

    fn take_row_to_ans(&mut self, row: usize, col: usize) {
        assert!(self.cover[row][col], "row going to ans must be alive");
        {
            let (i, j, d) = decode_row_id(row);
            self.board[i][j] = d as i32;
        }
        let j0 = col;
        let mut j = j0;
        loop {
            // remove all other rows with 1s in the same column
            let i0 = row;
            let mut i = row;
            loop {
                i = self.grid[i][j].down;
                if i == i0 {
                    break;
                }
                self.remove_row(i, j);
            }
            self.kill_col(j);
            j = self.grid[row][j].right;
            if j == j0 {
                break;
            }
        }
        self.remove_row(row, col);
    }

    fn try_take_row(&mut self, row: usize, col: usize) -> bool {
        let old_cnt_ops = self.ops.len();
        self.take_row_to_ans(row, col);
        if self.go() {
            return true;
        }
        while self.ops.len() > old_cnt_ops {
            self.pop_op();
        }
        false
    }

    fn go(&mut self) -> bool {
        if self.alive_cols_list_head.is_none() {
            return true;
        }

        let mut min_col = None;
        {
            let col0 = self.alive_cols_list_head.unwrap();
            let mut col = col0;
            loop {
                if min_col.is_none()
                    || self.cnt_who_cover_this_col[col]
                        < self.cnt_who_cover_this_col[min_col.unwrap()]
                {
                    min_col = Some(col);
                }
                col = self.alive_cols_list[col].right;
                if col == col0 {
                    break;
                }
            }
        }
        assert!(!min_col.is_none());
        let min_col = min_col.unwrap();
        if self.cnt_who_cover_this_col[min_col] == 0 {
            return false;
        }

        let col = min_col;
        assert!(!self.is_col_killed[col]);
        let row0 = self.head_of_col[col].unwrap();
        assert!(self.cover[row0][col]);
        let mut row = row0;
        loop {
            assert!(
                self.cover[row][col],
                "bad take_row cand: row={}, col={}",
                row, col
            );
            if self.try_take_row(row, col) {
                return true;
            }
            assert!(self.cover[row][col]);
            row = self.grid[row][col].up;
            if row == row0 {
                break;
            }
        }

        false
    }

    fn new(b: &Vec<Vec<char>>) -> Self {
        let mut this = Solver {
            grid: vec![[Node::new(); COLS]; ROWS],
            head_of_col: [None; COLS],
            alive_cols_list: [Node::new(); COLS],
            alive_cols_list_head: None,
            cnt_who_cover_this_col: [0; COLS],
            board: [[-1; 9]; 9],
            ops: Vec::new(),
            cover: vec![vec![false; COLS]; ROWS],
            is_col_killed: vec![false; COLS],
        };
        for i in 0..9 {
            for j in 0..9 {
                this.board[i][j] = if b[i][j] == '.' {
                    -1
                } else {
                    (b[i][j].to_digit(10).unwrap() - 1) as i32
                }
            }
        }
        this
    }

    fn solve(&mut self) -> [[i32; 9]; 9] {
        // fill rows x cols matrix with conditions
        for row in 0..ROWS {
            let mut col_offset = 0;
            let (i, j, d) = decode_row_id(row);

            // only one digit in each cell
            let col = col_offset + i * 9 + j;
            self.cover[row][col] = true;
            col_offset += 9 * 9;

            // only one instance of d in each column
            let col = col_offset + j * 9 + d;
            self.cover[row][col] = true;
            col_offset += 9 * 9;

            // only one instance of d in each row
            let col = col_offset + i * 9 + d;
            self.cover[row][col] = true;
            col_offset += 9 * 9;

            // only one instance of d in each subcell
            let ii = i / 3;
            let jj = j / 3;
            let cc = ii * 3 + jj;
            let col = col_offset + cc * 9 + d;
            self.cover[row][col] = true;
            // col_offset += 9 * 9;
        }

        // build the grid
        for j in 0..COLS {
            let mut rs = Vec::new();
            for i in 0..ROWS {
                if self.cover[i][j] {
                    rs.push(i);
                }
            }
            assert!(!rs.is_empty());
            self.cnt_who_cover_this_col[j] = rs.len();
            for pos in 0..rs.len() {
                let i = rs[pos];
                let i_prev = rs[(pos + rs.len() - 1) % rs.len()];
                let i_next = rs[(pos + 1) % rs.len()];
                self.grid[i][j].up = i_prev;
                self.grid[i][j].down = i_next;
                self.head_of_col[j] = Some(i);
            }
        }
        for i in 0..ROWS {
            let mut cs = Vec::new();
            for j in 0..COLS {
                if self.cover[i][j] {
                    cs.push(j);
                }
            }
            assert!(!cs.is_empty());
            for pos in 0..cs.len() {
                let j = cs[pos];
                let j_prev = cs[(pos + cs.len() - 1) % cs.len()];
                let j_next = cs[(pos + 1) % cs.len()];
                self.grid[i][j].left = j_prev;
                self.grid[i][j].right = j_next;
            }
        }
        for j in 0..COLS {
            let j_prev = (j + COLS - 1) % COLS;
            let j_next = (j + 1) % COLS;
            self.alive_cols_list[j].left = j_prev;
            self.alive_cols_list[j].right = j_next;
        }
        self.alive_cols_list_head = Some(0);

        for i in 0..9 {
            for j in 0..9 {
                let d = self.board[i][j];
                if d == -1 {
                    continue;
                }
                assert!(0 <= d && d < 9);
                let row = i * 9 * 9 + j * 9 + d as usize;
                let col = (0..COLS).find(|&j| self.cover[row][j]).unwrap();
                assert!(self.cover[row][col]);
                self.take_row_to_ans(row, col);
            }
        }
        assert!(self.go());

        self.board.clone()
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut solver = Solver::new(board);
        let ans = solver.solve();

        for i in 0..9 {
            for j in 0..9 {
                let d = ans[i][j];
                assert!(0 <= d && d < 9);
                board[i][j] = std::char::from_digit((d + 1) as u32, 10).unwrap();
            }
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

fn to_vec_vec_char(b: &[[&str; 9]; 9]) -> Vec<Vec<char>> {
    let mut res = Vec::<Vec<char>>::new();
    for i in 0..9 {
        let mut row = Vec::<char>::new();
        for j in 0..9 {
            row.push(b[i][j].as_bytes()[0] as char);
        }
        res.push(row);
    }
    res
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    fn case() {
        // let input = [
        //     ["5", "3", ".", ".", "7", ".", ".", ".", "."],
        //     ["6", ".", ".", "1", "9", "5", ".", ".", "."],
        //     [".", "9", "8", ".", ".", ".", ".", "6", "."],
        //     ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
        //     ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
        //     ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
        //     [".", "6", ".", ".", ".", ".", "2", "8", "."],
        //     [".", ".", ".", "4", "1", "9", ".", ".", "5"],
        //     [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        // ];
        // let input = [
        //     [".", ".", "9", "7", "4", "8", ".", ".", "."],
        //     ["7", ".", ".", ".", ".", ".", ".", ".", "."],
        //     [".", "2", ".", "1", ".", "9", ".", ".", "."],
        //     [".", ".", "7", ".", ".", ".", "2", "4", "."],
        //     [".", "6", "4", ".", "1", ".", "5", "9", "."],
        //     [".", "9", "8", ".", ".", ".", "3", ".", "."],
        //     [".", ".", ".", "8", ".", "3", ".", "2", "."],
        //     [".", ".", ".", ".", ".", ".", ".", ".", "6"],
        //     [".", ".", ".", "2", "7", "5", "9", ".", "."],
        // ];
        let input = [
            [".", ".", "9", "7", "4", "8", ".", ".", "."],
            ["7", ".", ".", ".", ".", ".", ".", ".", "."],
            [".", "2", ".", "1", ".", "9", ".", ".", "."],
            [".", ".", "7", ".", ".", ".", "2", "4", "."],
            [".", "6", "4", ".", "1", ".", "5", "9", "."],
            [".", "9", "8", ".", ".", ".", "3", ".", "."],
            [".", ".", ".", "8", ".", "3", ".", "2", "."],
            [".", ".", ".", ".", ".", ".", ".", ".", "6"],
            [".", ".", ".", "2", "7", "5", "9", ".", "."],
        ];
        let expected = [
            ["5", "3", "4", "6", "7", "8", "9", "1", "2"],
            ["6", "7", "2", "1", "9", "5", "3", "4", "8"],
            ["1", "9", "8", "3", "4", "2", "5", "6", "7"],
            ["8", "5", "9", "7", "6", "1", "4", "2", "3"],
            ["4", "2", "6", "8", "5", "3", "7", "9", "1"],
            ["7", "1", "3", "9", "2", "4", "8", "5", "6"],
            ["9", "6", "1", "5", "3", "7", "2", "8", "4"],
            ["2", "8", "7", "4", "1", "9", "6", "3", "5"],
            ["3", "4", "5", "2", "8", "6", "1", "7", "9"],
        ];
        let mut board = to_vec_vec_char(&input);
        Solution::solve_sudoku(&mut board);
        assert_eq!(board.len(), 9);
        for i in 0..9 {
            assert_eq!(board[i].len(), 9);
            for j in 0..9 {
                assert_eq!(board[i][j], expected[i][j].as_bytes()[0] as char);
            }
        }
    }
}
