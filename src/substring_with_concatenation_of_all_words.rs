//! Solution for https://leetcode.com/problems/substring-with-concatenation-of-all-words
//! 30. Substring with Concatenation of All Words

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut need_cnt = HashMap::<String, i32>::new();
        for w in &words {
            *need_cnt.entry(w.to_string()).or_insert(0) += 1;
        }
        let distinct_words = need_cnt.len();

        let n = s.len();
        let mut ans = Vec::<i32>::new();
        let m = words[0].len();
        let cnt_w = words.len();
        for rem in 0..m {
            if rem + m * cnt_w > n {
                break;
            }
            let mut cur_cnt = HashMap::<String, i32>::new();
            let mut update = |word: &str, is_add: bool, cnt_ok: &mut usize| {
                if !need_cnt.contains_key(word) {
                    return;
                }
                if !cur_cnt.contains_key(word) {
                    cur_cnt.insert(word.to_string(), 0);
                }
                if need_cnt.get(word) == cur_cnt.get(word) {
                    *cnt_ok -= 1;
                }
                *cur_cnt.get_mut(word).unwrap() += if is_add { 1 } else { -1 };
                if need_cnt.get(word) == cur_cnt.get(word) {
                    *cnt_ok += 1;
                }
            };

            let mut cnt_ok = 0;
            for i in 0..cnt_w {
                let pos = rem + i * m;
                update(&s[pos..pos + m], true, &mut cnt_ok);
            }
            for st in (rem..=n - m * cnt_w).step_by(m) {
                if cnt_ok == distinct_words {
                    ans.push(st as i32);
                }
                update(&s[st..st + m], false, &mut cnt_ok);
                if st + m * cnt_w + m <= n {
                    update(&s[st + m * cnt_w..st + m * cnt_w + m], true, &mut cnt_ok);
                }
            }
        }
        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("barfoothefoobarman", vec!["foo".into(),"bar".into()], vec![0,9])]
    #[case("wordgoodgoodgoodbestword", vec!["word".into(),"good".into(),"best".into(),"word".into()], vec![])]
    #[case("barfoofoobarthefoobarman", vec!["bar".into(),"foo".into(),"the".into()], vec![6,9,12])]
    fn case(#[case] s: String, #[case] words: Vec<String>, #[case] expected: Vec<i32>) {
        let actual = Solution::find_substring(s, words);
        assert_eq!(actual, expected);
    }
}
