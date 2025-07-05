//! Solution for https://leetcode.com/problems/stickers-to-spell-word
//! 691. Stickers to Spell Word

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let stickers = to_u8_vec(&stickers);
        let target = to_u8(&target);
        let n = stickers.len();
        let m = target.len();

        // compress letters to only those appearing in target
        let mut letters = target.clone();
        letters.sort();
        letters.dedup();
        let A = letters.len();

        let ch_to_code = |ch: u8| -> Option<usize> {
            if let Ok(pos) = letters.binary_search(&ch) {
                Some(pos)
            } else {
                None
            }
        };

        let str_to_hist = |s: &Vec<u8>| -> Vec<usize> {
            let mut hist = vec![0; A];
            for &ch in s {
                let pos = ch_to_code(ch);
                if pos.is_some() {
                    hist[pos.unwrap()] += 1;
                }
            }
            hist
        };

        let stickers: Vec<_> = stickers.iter().map(str_to_hist).collect();
        let target: Vec<_> = target.iter().map(|&ch| ch_to_code(ch).unwrap()).collect();

        let INF = m + 1;
        let mut dp = vec![INF; 1 << m];
        dp[0] = 0;
        for sticker in &stickers {
            for covered in (0..1 << m).rev() {
                let cur_dp = dp[covered];
                if cur_dp >= INF {
                    continue;
                }
                let mut prev_nmask = 0;
                for mult in 1..=target.len() {
                    let mut nmask = covered;
                    let mut cnt = sticker.clone();
                    for c in &mut cnt {
                        *c *= mult;
                    }
                    for i in 0..m {
                        if ((covered >> i) & 1) == 0 {
                            let c = target[i];
                            if cnt[c] > 0 {
                                nmask |= 1 << i;
                                cnt[c] -= 1;
                            }
                        }
                    }
                    if nmask == prev_nmask {
                        break;
                    }
                    prev_nmask = nmask;
                    dp[nmask] = dp[nmask].min(cur_dp + mult);
                }
            }
        }

        let ans = dp[(1 << m) - 1];
        if ans > m {
            return -1;
        }
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
    #[case(vec!["with".into(),"example".into(),"science".into()], "thehat", 3)]
    #[case(vec!["notice".into(),"possible".into()], "basicbasic", -1)]
    fn case(#[case] stickers: Vec<String>, #[case] target: String, #[case] expected: i32) {
        let actual = Solution::min_stickers(stickers, target);
        assert_eq!(actual, expected);
    }
}
