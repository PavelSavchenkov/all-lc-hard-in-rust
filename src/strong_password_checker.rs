//! Solution for https://leetcode.com/problems/strong-password-checker
//! 420. Strong Password Checker

const A: usize = 26 * 2 + 10 + 2;
const MIN_LEN: usize = 6;
const MAX_LEN: usize = 20;

fn code(c: u8) -> usize {
    let u8_code = match c {
        b'a'..=b'z' => c - b'a',
        b'A'..=b'Z' => 26 + c - b'A',
        b'0'..=b'9' => 26 * 2 + c - b'0',
        b'.' => 26 * 2 + 10 + 0,
        b'!' => 26 * 2 + 10 + 1 + 0,
        _ => panic!("Wrong char: {}", c as char),
    };
    u8_code as usize
}

fn is_digit(c: usize) -> bool {
    26 * 2 <= c && c < 26 * 2 + 10
}

fn is_lowercase(c: usize) -> bool {
    c < 26
}

fn is_uppercase(c: usize) -> bool {
    26 <= c && c < 26 * 2
}

fn remin(x: &mut i32, y: i32) {
    if *x > y {
        *x = y;
    }
}

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let s = to_u8(&password);

        // len, i (in s), was lowercase, was uppercase, was digit, 0..=1 (c0 == c1 or not), c1
        let mut dp = vec![
            vec![vec![vec![vec![vec![vec![i32::MAX; A]; 2]; 2]; 2]; 2]; s.len() + 1];
            MAX_LEN + 1
        ];
        dp[0][0][0][0][0][0][0] = 0;

        let mut ch_candidates = Vec::new();
        for it in 0..2 {
            let upper = code(b'A') + it;
            ch_candidates.push(upper);
            let lower = code(b'a') + it;
            ch_candidates.push(lower);
            let digit = code(b'0') + it;
            ch_candidates.push(digit);
        }

        for len in 0..MAX_LEN {
            for i in 0..=s.len() {
                for was_lowercase in 0..2 {
                    for was_uppercase in 0..2 {
                        for was_digit in 0..2 {
                            for c0_eq_c1 in 0..2 {
                                for c1 in 0..A {
                                    let cur_dp = dp[len][i][was_lowercase][was_uppercase]
                                        [was_digit][c0_eq_c1][c1];
                                    if cur_dp == i32::MAX {
                                        continue;
                                    }

                                    if i < s.len() {
                                        let true_ch = code(s[i]);

                                        // delete from s
                                        remin(
                                            &mut dp[len][i + 1][was_lowercase][was_uppercase]
                                                [was_digit][c0_eq_c1][c1],
                                            cur_dp + 1,
                                        );

                                        ch_candidates.push(true_ch);
                                        // replace or keep from s
                                        for &ch in &ch_candidates {
                                            if len < 2 || c0_eq_c1 == 0 || c1 != ch {
                                                let mut new_was_lowercase = was_lowercase;
                                                if is_lowercase(ch) {
                                                    new_was_lowercase = 1;
                                                }
                                                let mut new_was_uppercase = was_uppercase;
                                                if is_uppercase(ch) {
                                                    new_was_uppercase = 1;
                                                }
                                                let mut new_was_digit = was_digit;
                                                if is_digit(ch) {
                                                    new_was_digit = 1;
                                                }
                                                let mut new_dp = cur_dp;
                                                if ch != true_ch {
                                                    new_dp += 1;
                                                }
                                                let mut new_c0_eq_c1 = 0;
                                                if ch == c1 {
                                                    new_c0_eq_c1 = 1;
                                                }
                                                remin(
                                                    &mut dp[len + 1][i + 1][new_was_lowercase]
                                                        [new_was_uppercase][new_was_digit]
                                                        [new_c0_eq_c1][ch],
                                                    new_dp,
                                                );
                                            }
                                        }
                                        ch_candidates.pop();
                                    }

                                    // insert new ch
                                    for &ch in &ch_candidates {
                                        if len < 2 || c0_eq_c1 == 0 || c1 != ch {
                                            let mut new_was_lowercase = was_lowercase;
                                            if is_lowercase(ch) {
                                                new_was_lowercase = 1;
                                            }
                                            let mut new_was_uppercase = was_uppercase;
                                            if is_uppercase(ch) {
                                                new_was_uppercase = 1;
                                            }
                                            let mut new_was_digit = was_digit;
                                            if is_digit(ch) {
                                                new_was_digit = 1;
                                            }
                                            let mut new_c0_eq_c1 = 0;
                                            if ch == c1 {
                                                new_c0_eq_c1 = 1;
                                            }
                                            remin(
                                                &mut dp[len + 1][i][new_was_lowercase]
                                                    [new_was_uppercase][new_was_digit]
                                                    [new_c0_eq_c1][ch],
                                                cur_dp + 1,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut ans = i32::MAX;
        for len in MIN_LEN..=MAX_LEN {
            for c0_eq_c1 in 0..2 {
                for c1 in 0..A {
                    let cur = dp[len][s.len()][1][1][1][c0_eq_c1][c1];
                    ans = ans.min(cur);
                }
            }
        }

        ans
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
    #[case("a", 5)]
    #[case("aA1", 3)]
    #[case("1337C0d3", 0)]
    fn case(#[case] password: String, #[case] expected: i32) {
        let actual = Solution::strong_password_checker(password);
        assert_eq!(actual, expected);
    }
}
