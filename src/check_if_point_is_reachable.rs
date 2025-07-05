//! Solution for https://leetcode.com/problems/check-if-point-is-reachable
//! 2543. Check if Point Is Reachable

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Solution {
    pub fn is_reachable(mut x: i32, mut y: i32) -> bool {
        while x % 2 == 0 {
            x /= 2;
        }
        while y % 2 == 0 {
            y /= 2;
        }
        gcd(x, y) == 1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(6, 9, false)]
    #[case(4, 7, true)]
    fn case(#[case] target_x: i32, #[case] target_y: i32, #[case] expected: bool) {
        let actual = Solution::is_reachable(target_x, target_y);
        assert_eq!(actual, expected);
    }
}
