//! Solution for https://leetcode.com/problems/string-compression-ii
//! 1531. String Compression II

fn len_num(num: usize) -> usize {
    if num <= 1 {
        return 0;
    }
    if num < 10 {
        return 1;
    }
    if num < 100 {
        return 2;
    }
    assert!(num < 1000);
    return 3;
}

fn remin(a: &mut usize, b: usize) {
    if *a > b {
        *a = b;
    }
}

const A: usize = 26;

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let s = to_u8(&s);
        let n = s.len();
        let k = k as usize;

        // deleted, last_c, cnt_c --> min len
        let mut dp = vec![vec![vec![usize::MAX; n + 1]; A]; k + 1];
        dp[0][0][0] = 0;
        for i in 0..n {
            let c = (s[i] - b'a') as usize;
            let mut ndp = vec![vec![vec![usize::MAX; n + 1]; A]; k + 1];
            for deleted in 0..=k {
                for last_c in 0..A {
                    for cnt_c in 0..=i {
                        let cur_dp = dp[deleted][last_c][cnt_c];
                        if cur_dp == usize::MAX {
                            continue;
                        }
                        // delete current
                        if deleted < k {
                            remin(&mut ndp[deleted + 1][last_c][cnt_c], cur_dp);
                        }
                        // keep current
                        // 1. new seq
                        if deleted == i {
                            remin(&mut ndp[deleted][c][1], cur_dp);
                        } else if last_c != c {
                            remin(&mut ndp[deleted][c][1], cur_dp + 1 + len_num(cnt_c));
                        } else {
                            // 2. continue last_c
                            remin(&mut ndp[deleted][last_c][cnt_c + 1], cur_dp);
                        }
                    }
                }
            }
            dp = ndp;
        }

        let mut ans = usize::MAX;
        for last_c in 0..A {
            for cnt_c in 0..=n {
                let cur_dp = dp[k][last_c][cnt_c];
                if cur_dp == usize::MAX {
                    continue;
                }
                let mut cand = cur_dp;
                if cnt_c > 0 {
                    cand += 1 + len_num(cnt_c);
                }
                ans = ans.min(cand);
            }
        }
        assert!(ans < usize::MAX);
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
    #[case("aaabcccd", 2, 4)]
    #[case("aabbaa", 2, 2)]
    #[case("aaaaaaaaaaa", 0, 3)]
    fn case(#[case] s: String, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::get_length_of_optimal_compression(s, k);
        assert_eq!(actual, expected);
    }
}
