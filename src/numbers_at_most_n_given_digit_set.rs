//! Solution for https://leetcode.com/problems/numbers-at-most-n-given-digit-set
//! 902. Numbers At Most N Given Digit Set

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let mut can = vec![false; 10];
        for dig in digits {
            let dig = dig.as_bytes()[0] - b'0';
            can[dig as usize] = true;
        }
        let mut n = (n + 1) as u32;

        let mut nn = Vec::new();
        while n > 0 {
            nn.push((n % 10) as usize);
            n /= 10;
        }
        nn.reverse();

        let mut dp = vec![vec![0; 2]; nn.len() + 1];
        dp[0][0] = 1;
        for i in 1..nn.len() {
            dp[i][1] = 1;
        }
        for i in 0..nn.len() {
            for less in 0..2 {
                for cur in 0..10 {
                    if !can[cur] {
                        continue;
                    }
                    let mut nless = less;
                    if less == 0 && cur > nn[i] {
                        continue;
                    }
                    if cur < nn[i] {
                        nless = 1;
                    }
                    dp[i + 1][nless] += dp[i][less];
                }
            }
        }
        dp[nn.len()][1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["1".into(),"3".into(),"5".into(),"7".into()], 100, 20)]
    #[case(vec!["1".into(),"4".into(),"9".into()], 1000000000, 29523)]
    #[case(vec!["7".into()], 8, 1)]
    fn case(#[case] digits: Vec<String>, #[case] n: i32, #[case] expected: i32) {
        let actual = Solution::at_most_n_given_digit_set(digits, n);
        assert_eq!(actual, expected);
    }
}
