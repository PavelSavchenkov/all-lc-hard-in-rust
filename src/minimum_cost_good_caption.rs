//! Solution for https://leetcode.com/problems/minimum-cost-good-caption
//! 3441. Minimum Cost Good Caption

const A: usize = 26;

impl Solution {
    pub fn min_cost_good_caption(caption: String) -> String {
        let s = to_u8(&caption);

        let change_cost = |i: usize, new_c: usize| -> usize {
            let old_c = (s[i] - b'a') as usize;
            (old_c as i32 - new_c as i32).abs() as usize
        };

        if s.len() <= 2 {
            return "".to_string();
        }

        let n = s.len();

        let mut dp = vec![vec![vec![usize::MAX; 4]; A]; n + 1];
        for c in 0..A {
            dp[n - 1][c][1] = change_cost(n - 1, c);
        }
        for i in (0..n - 1).rev() {
            for c in 0..A {
                for len in 1..=3 {
                    let cur_dp = dp[i + 1][c][len];
                    if cur_dp == usize::MAX {
                        continue;
                    }
                    for nc in 0..A {
                        if len < 3 && c != nc {
                            continue;
                        }
                        let nlen = if c == nc { (len + 1).min(3) } else { 1 };
                        let ndp = cur_dp + change_cost(i, nc);
                        dp[i][nc][nlen] = dp[i][nc][nlen].min(ndp);
                    }
                }
            }
        }

        let mut min_cost = usize::MAX;
        let mut min_c = 0;
        for c in 0..A {
            let cur_dp = dp[0][c][3];
            if cur_dp < min_cost {
                min_c = c;
                min_cost = cur_dp;
            }
        }

        let mut ans = vec![0; n];
        let mut c = min_c;
        let mut pref_len = 1;
        ans[0] = b'a' + c as u8;
        min_cost -= change_cost(0, c);
        for i in 1..n {
            let mut next_c = c;
            if pref_len >= 3 {
                let mut best_nc = A;
                for nc in 0..A {
                    let mut ok = dp[i][nc][3] == min_cost;
                    if nc == c {
                        for len in 1..=2 {
                            ok |= dp[i][c][len] == min_cost;
                        }
                    }
                    if ok {
                        best_nc = nc;
                        break;
                    }
                }
                assert!(best_nc < A);
                next_c = best_nc;
            }
            if c == next_c {
                pref_len += 1;
            } else {
                assert!(pref_len >= 3);
                pref_len = 1;
            }
            c = next_c;
            ans[i] = b'a' + next_c as u8;
            min_cost -= change_cost(i, c);
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
    #[case("cdcd", "cccc")]
    #[case("aca", "aaa")]
    #[case("bc", "")]
    fn case(#[case] caption: String, #[case] expected: String) {
        let actual = Solution::min_cost_good_caption(caption);
        assert_eq!(actual, expected);
    }
}
