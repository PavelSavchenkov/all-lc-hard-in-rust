//! Solution for https://leetcode.com/problems/nth-magical-number
//! 878. Nth Magical Number

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a / gcd(a, b)) * b
}

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let ab = lcm(a, b);
        let a = a as i64;
        let b = b as i64;
        let ab = ab as i64;

        let mut L = 0;
        let mut R = n as i64 * ab + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            let cnt = M / a + M / b - M / ab;
            if cnt >= n as i64 {
                R = M
            } else {
                L = M;
            }
        }
        (R % 1_000_000_007) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 2, 3, 2)]
    #[case(4, 2, 3, 6)]
    fn case(#[case] n: i32, #[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        let actual = Solution::nth_magical_number(n, a, b);
        assert_eq!(actual, expected);
    }
}
