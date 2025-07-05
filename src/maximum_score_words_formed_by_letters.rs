//! Solution for https://leetcode.com/problems/maximum-score-words-formed-by-letters
//! 1255. Maximum Score Words Formed by Letters

const A: usize = 26;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let words = to_u8_vec(&words);

        let mut cnt_letter = vec![0; A];
        for &letter in &letters {
            cnt_letter[(letter as u8 - b'a') as usize] += 1;
        }

        assert!(score.len() == A);

        let mut ans = 0;
        for mask in 1..1 << words.len() {
            let mut ok = true;
            let mut sum_score = 0;
            for it in 0..2 {
                for i in 0..words.len() {
                    if ((mask >> i) & 1) == 1 {
                        for ch in &words[i] {
                            let c = (ch - b'a') as usize;
                            if it == 0 {
                                cnt_letter[c] -= 1;
                                if cnt_letter[c] < 0 {
                                    ok = false;
                                }
                                sum_score += score[c];
                            } else {
                                cnt_letter[c] += 1;
                            }
                        }
                    }
                }
            }
            if ok {
                ans = ans.max(sum_score);
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
    #[case(vec!["dog".into(),"cat".into(),"dad".into(),"good".into()], todo!("["a","a","c","d","d","d","g","o","o"]"), vec![1,0,9,5,0,0,3,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0], 23)]
    #[case(vec!["xxxz".into(),"ax".into(),"bx".into(),"cx".into()], todo!("["z","a","b","c","x","x","x"]"), vec![4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,10], 27)]
    #[case(vec!["leetcode".into()], todo!("["l","e","t","c","o","d"]"), vec![0,0,1,1,1,0,0,0,0,0,0,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0], 0)]
    fn case(
        #[case] words: Vec<String>,
        #[case] letters: Vec<char>,
        #[case] score: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::max_score_words(words, letters, score);
        assert_eq!(actual, expected);
    }
}
