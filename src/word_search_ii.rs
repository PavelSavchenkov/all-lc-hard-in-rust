//! Solution for https://leetcode.com/problems/word-search-ii
//! 212. Word Search II

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

const MAX_LEN: usize = 10;
const di: [i32; 4] = [0, 0, -1, 1];
const dj: [i32; 4] = [-1, 1, 0, 0];

fn rec(
    g: &Vec<Vec<char>>,
    used: &mut Vec<Vec<bool>>,
    cur_word: &mut Vec<u8>,
    i: usize,
    j: usize,
    all_words: &mut Vec<u64>,
) {
    let n = g.len() as i32;
    let m = g[0].len() as i32;

    assert!(!used[i][j]);
    used[i][j] = true;
    cur_word.push(g[i][j] as u8);
    {
        let h = get_hash(&cur_word);
        all_words.push(h);
    }
    if cur_word.len() < MAX_LEN {
        for d in 0..4 {
            let ni = i as i32 + di[d];
            let nj = j as i32 + dj[d];
            if ni < 0 || ni >= n || nj < 0 || nj >= m {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if used[ni][nj] {
                continue;
            }
            rec(g, used, cur_word, ni, nj, all_words);
        }
    }
    cur_word.pop();
    used[i][j] = false;
}

impl Solution {
    pub fn find_words(g: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let n = g.len();
        let m = g[0].len();

        let mut all_words = Vec::<u64>::new();
        let mut used = vec![vec![false; m]; n];
        for i in 0..n {
            for j in 0..m {
                let mut cur_word = Vec::<u8>::new();
                rec(&g, &mut used, &mut cur_word, i, j, &mut all_words);
            }
        }

        all_words.sort();
        all_words.dedup();

        let mut ans = Vec::new();
        for word in &words {
            let h = get_hash(&word.as_bytes());
            let res = all_words.binary_search(&h);
            if res.is_ok() {
                ans.push(word.clone());
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
    #[case(vec![vec!['o','a','a','n'],vec!['e','t','a','e'],vec!['i','h','k','r'],vec!['i','f','l','v']], vec!["oath".into(),"pea".into(),"eat".into(),"rain".into()], vec!["eat".into(),"oath".into()])]
    #[case(vec![vec!['a','b'],vec!['c','d']], vec!["abcb".into()], vec![])]
    fn case(
        #[case] board: Vec<Vec<char>>,
        #[case] words: Vec<String>,
        #[case] expected: Vec<String>,
    ) {
        let actual = Solution::find_words(board, words);
        assert_eq!(actual, expected);
    }
}
