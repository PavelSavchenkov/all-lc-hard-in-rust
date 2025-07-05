//! Solution for https://leetcode.com/problems/freedom-trail
//! 514. Freedom Trail

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring = to_u8(&ring);
        let key = to_u8(&key);
        let n = ring.len();
        let m = key.len();

        let mut dp = vec![vec![usize::MAX; n]; m + 1];
        dp[0][0] = 0;

        for j in 0..m {
            for i in 0..n {
                let cur_dp = dp[j][i];
                if cur_dp == usize::MAX {
                    continue;
                }
                for dir in 0..2 {
                    for it in 0..n {
                        let ii = if dir == 1 {
                            (i + it) % n
                        } else {
                            (i + n - it) % n
                        };
                        if ring[ii] == key[j] {
                            dp[j + 1][ii] = dp[j + 1][ii].min(cur_dp + it);
                        }
                    }
                }
            }
        }

        let ans = *dp[m].iter().min().unwrap() + m;
        ans as i32
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
    #[case("godding", "gd", 4)]
    #[case("godding", "godding", 13)]
    fn case(#[case] ring: String, #[case] key: String, #[case] expected: i32) {
        let actual = Solution::find_rotate_steps(ring, key);
        assert_eq!(actual, expected);
    }
}
