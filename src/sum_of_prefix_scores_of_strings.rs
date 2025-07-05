//! Solution for https://leetcode.com/problems/sum-of-prefix-scores-of-strings
//! 2416. Sum of Prefix Scores of Strings

const A: usize = 26;

fn code(ch: u8) -> usize {
    (ch - b'a') as usize
}

type TriePtr = Option<Box<Trie>>;

struct Trie {
    cnt_terminal: usize,
    to: Vec<TriePtr>,
}

impl Trie {
    fn new() -> Self {
        Self {
            cnt_terminal: 0,
            to: (0..A).map(|_| None).collect(),
        }
    }

    fn add_word(&mut self, word: &Vec<u8>) {
        let mut t = self;
        for &ch in word {
            let c = code(ch);
            t = t.to[c].get_or_insert_with(|| Box::new(Trie::new()));
            t.cnt_terminal += 1;
        }
    }

    fn get_sum(&self, word: &Vec<u8>) -> usize {
        let mut sum = 0;
        let mut t = self;
        for &ch in word {
            let c = code(ch);
            let next = &t.to[c];
            if next.is_none() {
                break;
            }
            t = next.as_ref().unwrap();
            sum += t.cnt_terminal;
        }
        sum
    }
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let words = to_u8_vec(&words);

        let mut root = Trie::new();
        for word in &words {
            root.add_word(&word);
        }

        let mut ans = Vec::new();
        for word in &words {
            ans.push(root.get_sum(&word) as i32);
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
    #[case(vec!["abc".into(),"ab".into(),"bc".into(),"b".into()], vec![5,4,3,2])]
    #[case(vec!["abcd".into()], vec![4])]
    fn case(#[case] words: Vec<String>, #[case] expected: Vec<i32>) {
        let actual = Solution::sum_prefix_scores(words);
        assert_eq!(actual, expected);
    }
}
