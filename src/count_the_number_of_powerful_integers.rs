//! Solution for https://leetcode.com/problems/count-the-number-of-powerful-integers
//! 2999. Count the Number of Powerful Integers

fn solve(mut up: i64, limit: usize, s: &Vec<u8>) -> i64 {
    let mut digits = Vec::new();
    while up > 0 {
        digits.push((up % 10) as usize);
        up /= 10;
    }
    digits.reverse();

    if s.len() > digits.len() {
        return 0;
    }

    let mut dp = vec![0; 2];
    dp[0] = 1;
    for i in 0..digits.len() {
        let mut from = 0;
        let mut to = limit;
        if i >= digits.len() - s.len() {
            from = (s[i - (digits.len() - s.len())] - b'0') as usize;
            to = from;
        }
        let mut ndp = vec![0; 2];
        for less in 0..2 {
            let cur_dp = dp[less];
            for dig in from..=to {
                if less == 0 && dig > digits[i] {
                    continue;
                }
                let mut nless = less;
                if dig < digits[i] {
                    nless = 1;
                }
                ndp[nless] += cur_dp;
            }
        }
        dp = ndp;
    }
    dp[1]
}

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let s = to_u8(&s);
        solve(finish + 1, limit as usize, &s) - solve(start, limit as usize, &s)
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
    #[case(1, 6000, 4, "124", 5)]
    #[case(15, 215, 6, "10", 2)]
    #[case(1000, 2000, 4, "3000", 0)]
    fn case(
        #[case] start: i64,
        #[case] finish: i64,
        #[case] limit: i32,
        #[case] s: String,
        #[case] expected: i64,
    ) {
        let actual = Solution::number_of_powerful_int(start, finish, limit, s);
        assert_eq!(actual, expected);
    }
}
