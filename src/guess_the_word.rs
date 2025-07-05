//! Solution for https://leetcode.com/problems/guess-the-word
//! 843. Guess the Word

/**
 * // This is the Master's API interface.
 * // You should not implement it, or speculate about its implementation
 * struct Master;
 * impl Master {
 *     fn guess(word:String)->int;
 * };
 */

const L: usize = 6;

impl Solution {
    pub fn find_secret_word(words: Vec<String>, master: &Master) {
        let n = words.len();

        let mut dist = vec![vec![L; n]; n];
        for i in 0..n {
            for j in 0..i {
                let w0 = to_u8(&words[i]);
                let w1 = to_u8(&words[j]);
                assert!(w0.len() == L);
                assert!(w1.len() == L);
                let d = (0..L).filter(|&k| w0[k] == w1[k]).count();
                dist[i][j] = d;
                dist[j][i] = d;
            }
        }

        let mut alive = vec![true; n];
        let mut used = vec![false; n];
        loop {
            let mut best_i = 0;
            let mut best_max_group = n;
            let mut cnt_alive = 0;
            for i in 0..n {
                if alive[i] {
                    cnt_alive += 1;
                }
                if used[i] {
                    continue;
                }
                let mut cnt = vec![0; L + 1];
                for j in 0..n {
                    if alive[j] {
                        cnt[dist[i][j]] += 1;
                    }
                }
                let max_group_size = *cnt.iter().max().unwrap();
                if max_group_size < best_max_group {
                    best_max_group = max_group_size;
                    best_i = i;
                }
            }
            assert!(cnt_alive > 0);
            if cnt_alive == 1 {
                for i in 0..n {
                    if alive[i] {
                        best_i = i;
                    }
                }
            }
            used[best_i] = true;
            let answer = master.guess(words[best_i].clone());
            assert!(answer != -1);
            let answer = answer as usize;
            if answer == L {
                break;
            }
            let mut was_unalive = false;
            for j in 0..n {
                if dist[best_i][j] != answer && alive[j] {
                    was_unalive = true;
                    alive[j] = false;
                }
            }
            assert!(was_unalive);
        }
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

    //     #[rstest]
    //     #[case(/* Error: expected number of augments (2) to match the number of lines downloaded (3)
    // Raw example test cases:
    // "acckzz"
    // ["acckzz","ccbazz","eiowzz","abcczz"]
    // 10 */)]
    //     #[case(/* Error: expected number of augments (2) to match the number of lines downloaded (3)
    // Raw example test cases:
    // "hamada"
    // ["hamada","khaled"]
    // 10 */)]
    //     fn case(#[case] words: Vec<String>, #[case] master: &Master) {
    //         let actual = Solution::find_secret_word(words, master);
    //         assert_eq!(actual, expected);
    //     }
}
