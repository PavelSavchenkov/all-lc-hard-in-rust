//! Solution for https://leetcode.com/problems/largest-multiple-of-three
//! 1363. Largest Multiple of Three

impl Solution {
    pub fn largest_multiple_of_three(mut digits: Vec<i32>) -> String {
        let n = digits.len();
        digits.sort();
        digits.reverse();

        let mut dp = vec![vec![i32::MIN; 3]; n + 1];
        dp[n][0] = 0;
        for i in (0..n).rev() {
            for rem in 0..3 {
                dp[i][rem] = dp[i + 1][rem];
                let prev_rem = (rem as i32 - digits[i] % 3 + 3) % 3;
                dp[i][rem] = dp[i][rem].max(1 + dp[i + 1][prev_rem as usize]);
            }
        }

        let mut len = dp[0][0];
        if len == 0 {
            return String::new();
        }
        let mut ans = Vec::new();
        let mut rem = 0;
        for i in 0..n {
            let prev_rem = (rem as i32 - digits[i] % 3 + 3) % 3;
            if dp[i + 1][prev_rem as usize] + 1 == len {
                ans.push(b'0' + digits[i] as u8);
                rem = prev_rem;
                len -= 1;
            }
        }
        assert!(len == 0);

        if ans[0] == b'0' {
            ans.truncate(1);
        }
        from_u8(&ans)
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
    #[case(vec![8,1,9], "981")]
    #[case(vec![8,6,7,1,0], "8760")]
    #[case(vec![1], "")]
    fn case(#[case] digits: Vec<i32>, #[case] expected: String) {
        let actual = Solution::largest_multiple_of_three(digits);
        assert_eq!(actual, expected);
    }
}
