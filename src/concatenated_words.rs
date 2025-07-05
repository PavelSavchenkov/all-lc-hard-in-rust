//! Solution for https://leetcode.com/problems/concatenated-words
//! 472. Concatenated Words

const A: usize = 26;

fn code(ch: u8) -> usize {
    (ch - b'a') as usize
}

type TriePtr = Option<Box<Trie>>;

#[derive(Default)]
struct Trie {
    to: Vec<TriePtr>,
    is_terminal: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            to: (0..A).map(|i| None).collect(),
            is_terminal: false,
        }
    }

    fn add_word(&mut self, word: &Vec<u8>) {
        let mut t = self;
        for &ch in word {
            let c = code(ch);
            t = t.to[c].get_or_insert_with(|| Box::new(Trie::new()));
        }
        t.is_terminal = true;
    }
}

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let words = to_u8_vec(&words);

        let mut root = Trie::new();
        for w in &words {
            root.add_word(&w);
        }

        let mut ans = Vec::new();
        for w in &words {
            let n = w.len();
            let mut dp = vec![false; n + 1];
            dp[0] = true;
            for i in 0..n {
                if !dp[i] {
                    continue;
                }
                let mut t = &root;
                for j in i..n {
                    let c = code(w[j]);
                    if t.to[c].is_none() {
                        break;
                    }
                    t = t.to[c].as_ref().unwrap();
                    if t.is_terminal {
                        if !(i == 0 && j + 1 == n) {
                            dp[j + 1] = true;
                        }
                    }
                }
            }
            if dp[n] {
                ans.push(from_u8(w));
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
    #[case(vec!["cat".into(),"cats".into(),"catsdogcats".into(),"dog".into(),"dogcatsdog".into(),"hippopotamuses".into(),"rat".into(),"ratcatdogcat".into()], vec!["catsdogcats".into(),"dogcatsdog".into(),"ratcatdogcat".into()])]
    #[case(vec!["cat".into(),"dog".into(),"catdog".into()], vec!["catdog".into()])]
    fn case(#[case] words: Vec<String>, #[case] expected: Vec<String>) {
        let actual = Solution::find_all_concatenated_words_in_a_dict(words);
        assert_eq!(actual, expected);
    }
}
