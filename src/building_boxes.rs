//! Solution for https://leetcode.com/problems/building-boxes
//! 1739. Building Boxes

impl Solution {
    pub fn minimum_boxes(n: i32) -> i32 {
        if n <= 3 {
            return n;
        }

        let mut cnt_2d = 1;
        let mut cnt_3d = 1;
        let mut cnt_1d = 1;
        loop {
            cnt_1d += 1;
            cnt_2d += cnt_1d;
            cnt_3d += cnt_2d;
            if cnt_3d >= n {
                let remaining = n - (cnt_3d - cnt_2d);
                let mut cand_2d = 0;
                for cand_1d in 1..cnt_1d {
                    cand_2d += cand_1d;
                    if cand_2d >= remaining {
                        return (cnt_2d - cnt_1d) + cand_1d;
                    }
                }
                return cnt_2d;
            }
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, 3)]
    #[case(4, 3)]
    #[case(10, 6)]
    #[case(100, 34)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::minimum_boxes(n);
        assert_eq!(actual, expected);
    }
}
