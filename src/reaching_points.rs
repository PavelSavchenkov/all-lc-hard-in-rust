//! Solution for https://leetcode.com/problems/reaching-points
//! 780. Reaching Points

fn solve(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
    if sx > tx || sy > ty {
        return false;
    }
    if sx == tx && sy == ty {
        return true;
    }
    if tx == ty {
        return false;
    }
    if tx < ty {
        return solve(sy, sx, ty, tx);
    }
    // tx -= ty
    if sy == ty && (tx - sx) % ty == 0 {
        return true;
    }
    solve(sx, sy, tx % ty, ty)
}

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        solve(sx, sy, tx, ty)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 1, 3, 5, true)]
    #[case(1, 1, 2, 2, false)]
    #[case(1, 1, 1, 1, true)]
    fn case(
        #[case] sx: i32,
        #[case] sy: i32,
        #[case] tx: i32,
        #[case] ty: i32,
        #[case] expected: bool,
    ) {
        let actual = Solution::reaching_points(sx, sy, tx, ty);
        assert_eq!(actual, expected);
    }
}
