//! Solution for https://leetcode.com/problems/number-of-valid-words-for-each-puzzle
//! 1178. Number of Valid Words for Each Puzzle

use std::collections::HashMap;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let words = to_u8_vec(&words);
        let puzzles = to_u8_vec(&puzzles);

        let mut cnt_mask = HashMap::new();
        for word in &words {
            let mut mask = 0;
            for ch in word {
                let c = (ch - b'a') as usize;
                mask |= 1 << c;
            }
            *cnt_mask.entry(mask).or_insert(0) += 1;
        }

        let mut ans = Vec::new();
        for i in 0..puzzles.len() {
            let p = &puzzles[i];
            let mut f = usize::MAX;
            let mut mask = 0;
            for ch in p {
                let c = (ch - b'a') as usize;
                if f == usize::MAX {
                    f = c;
                }
                mask |= 1 << c;
            }
            mask ^= 1 << f;
            let mut cur_mask = mask;
            let mut cur_ans = 0;
            loop {
                cur_ans += *cnt_mask.entry(cur_mask | (1 << f)).or_default();
                if cur_mask == 0 {
                    break;
                }
                cur_mask = (cur_mask - 1) & mask;
            }
            ans.push(cur_ans as i32);
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
    #[case(vec!["aaaa".into(),"asas".into(),"able".into(),"ability".into(),"actt".into(),"actor".into(),"access".into()], vec!["aboveyz".into(),"abrodyz".into(),"abslute".into(),"absoryz".into(),"actresz".into(),"gaswxyz".into()], vec![1,1,3,2,4,0])]
    #[case(vec!["apple".into(),"pleas".into(),"please".into()], vec!["aelwxyz".into(),"aelpxyz".into(),"aelpsxy".into(),"saelpxy".into(),"xaelpsy".into()], vec![0,1,3,2,0])]
    fn case(#[case] words: Vec<String>, #[case] puzzles: Vec<String>, #[case] expected: Vec<i32>) {
        let actual = Solution::find_num_of_valid_words(words, puzzles);
        assert_eq!(actual, expected);
    }
}
