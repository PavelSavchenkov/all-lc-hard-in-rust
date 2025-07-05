//! Solution for https://leetcode.com/problems/race-car
//! 818. Race Car

use std::collections::VecDeque;

impl Solution {
    pub fn racecar(target: i32) -> i32 {
        // [-M; M]
        let M = target * 4;
        let mut log = 0;
        while (1 << log) < M {
            log += 1;
        }
        log += 1;

        let mut dp = vec![vec![vec![usize::MAX; 2]; log]; 2 * M as usize + 1];
        dp[M as usize][0][1] = 0;
        let mut q = VecDeque::new();
        q.push_front((0 as i32, 0 as usize, 1 as usize));
        while !q.is_empty() {
            let (x, pw, is_pos) = q.pop_front().unwrap();
            let cur_dp = dp[(x + M) as usize][pw][is_pos];
            assert!(cur_dp < usize::MAX);

            if x == target {
                return cur_dp as i32;
            }

            // continue moving
            {
                let coef = if is_pos == 1 { 1 } else { -1 };
                let next_x = x + coef * (1 << pw) as i32;
                if -M <= next_x && next_x <= M && pw + 1 < log {
                    if dp[(next_x + M) as usize][pw + 1][is_pos] > cur_dp + 1 {
                        dp[(next_x + M) as usize][pw + 1][is_pos] = cur_dp + 1;
                        q.push_back((next_x, pw + 1, is_pos));
                    }
                }
            }

            // reverse
            if dp[(x + M) as usize][0][1 - is_pos] > cur_dp + 1 {
                dp[(x + M) as usize][0][1 - is_pos] = cur_dp + 1;
                q.push_back((x, 0, 1 - is_pos));
            }
        }
        unreachable!()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, 2)]
    #[case(6, 5)]
    fn case(#[case] target: i32, #[case] expected: i32) {
        let actual = Solution::racecar(target);
        assert_eq!(actual, expected);
    }
}
