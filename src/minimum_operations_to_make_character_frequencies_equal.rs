//! Solution for https://leetcode.com/problems/minimum-operations-to-make-character-frequencies-equal
//! 3389. Minimum Operations to Make Character Frequencies Equal

const A: usize = 26;

impl Solution {
    pub fn make_string_good(s: String) -> i32 {
        let s = to_u8(&s);
        let n = s.len();

        let mut cnt = vec![0; A];
        for &ch in &s {
            let c = (ch - b'a') as usize;
            cnt[c] += 1;
        }

        eprintln!("cnt = {:#?}", cnt);

        let mut ans = i32::MAX;
        for target in 0..=n as i32 {
            let mut dp = vec![i32::MAX; 2];
            dp[0] = cnt[0];
            dp[1] = (cnt[0] - target).abs();
            for c in 1..A {
                let mut ndp = vec![i32::MAX; 2];
                for cur_dir in 0..2 {
                    let cur_steps = if cur_dir == 0 {
                        cnt[c]
                    } else {
                        (cnt[c] - target).abs()
                    };
                    for prev_dir in 0..2 {
                        let prev_dp = dp[prev_dir];
                        assert!(prev_dp < i32::MAX);
                        let mut new = prev_dp + cur_steps;
                        if prev_dir == 0 && cur_dir == 1 && cnt[c] < target {
                            let remove = cnt[c - 1].min(target - cnt[c]);
                            new -= remove;
                        } else if prev_dir == 1
                            && cnt[c - 1] > target
                            && cur_dir == 1
                            && cnt[c] < target
                        {
                            let remove = (cnt[c - 1] - target).min(target - cnt[c]);
                            new -= remove;
                        }
                        ndp[cur_dir] = ndp[cur_dir].min(new);
                    }
                }
                dp = ndp;
            }
            let cur = dp[0].min(dp[1]);
            ans = ans.min(cur);
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
    #[case("acab", 1)]
    #[case("wddw", 0)]
    #[case("aaabbc", 2)]
    #[case("accdddddbebbabbe", 5)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::make_string_good(s);
        assert_eq!(actual, expected);
    }
}
