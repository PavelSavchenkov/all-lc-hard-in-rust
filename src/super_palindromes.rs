//! Solution for https://leetcode.com/problems/super-palindromes
//! 906. Super Palindromes

fn is_pal(mut x: u64) -> bool {
    let mut ds = Vec::new();
    while x > 0 {
        ds.push(x % 10);
        x /= 10;
    }
    let mut ds_rev = ds.clone();
    ds_rev.reverse();
    ds == ds_rev
}

impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let left: u64 = left.parse().expect("Valid number");
        let right: u64 = right.parse().expect("Valid number");

        let mut ans = 0;
        for len in 1..=9 {
            let half = (len + 1) / 2;
            let start = u64::pow(10, half - 1);
            let end = u64::pow(10, half) - 1;
            for x in start..=end {
                let mut x_full = x;
                let mut tmp = x;
                if len % 2 == 1 {
                    tmp /= 10;
                }
                while tmp > 0 {
                    x_full *= 10;
                    x_full += tmp % 10;
                    tmp /= 10;
                }
                let x2 = x_full * x_full;
                if left <= x2 && x2 <= right && is_pal(x2) {
                    ans += 1;
                }
            }
        }
        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("4", "1000", 4)]
    #[case("1", "2", 1)]
    fn case(#[case] left: String, #[case] right: String, #[case] expected: i32) {
        let actual = Solution::superpalindromes_in_range(left, right);
        assert_eq!(actual, expected);
    }
}
