//! Solution for https://leetcode.com/problems/verbal-arithmetic-puzzle
//! 1307. Verbal Arithmetic Puzzle

struct Equation {
    summands: Vec<Vec<u8>>,
    result: Vec<u8>,
}

impl Equation {
    fn map_word(word: &Vec<u8>, mapping: &Vec<u32>) -> u32 {
        let mut pow10 = 1;
        let mut res = 0;
        for &ch in word.iter().rev() {
            let c = (ch - b'A') as usize;
            let val = mapping[c];
            res += pow10 * val;
            pow10 *= 10;
        }
        res
    }

    fn check(&self, mapping: &Vec<u32>) -> bool {
        let mut sum = 0;
        for summand in &self.summands {
            let cur_val = Self::map_word(summand, mapping);
            sum += cur_val;
        }
        let res = Self::map_word(&self.result, mapping);
        sum == res
    }
}

const A: usize = 26;

fn go(
    i: usize,
    letters: &Vec<u8>,
    mapping: &mut Vec<u32>,
    banned_from_zero: &Vec<bool>,
    used_digit: &mut Vec<bool>,
    equation: &Equation,
) -> bool {
    if i == letters.len() {
        return equation.check(mapping);
    }

    let c = (letters[i] - b'A') as usize;

    // todo: do not assign zero to letters which appear as the first letter in some word
    for dig in 0..10 {
        if dig == 0 && banned_from_zero[c] {
            continue;
        }
        if !used_digit[dig] {
            mapping[c] = dig as u32;
            used_digit[dig] = true;
            if go(
                i + 1,
                letters,
                mapping,
                banned_from_zero,
                used_digit,
                equation,
            ) {
                return true;
            }
            used_digit[dig] = false;
        }
    }
    false
}

impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let words = to_u8_vec(&words);
        let result = to_u8(&result);

        let mut letters = Vec::new();
        let mut letters_non_zero = Vec::new();
        for word in &words {
            for &ch in word {
                letters.push(ch);
            }
            if word.len() > 1 {
                letters_non_zero.push(word[0]);
            }
        }
        for &ch in &result {
            letters.push(ch);
        }
        if result.len() > 1 {
            letters_non_zero.push(result[0]);
        }
        letters.sort();
        letters.dedup();
        letters_non_zero.sort();
        letters_non_zero.dedup();

        let equation = Equation {
            summands: words.clone(),
            result: result.clone(),
        };

        let mut banned_from_zero = vec![false; A];
        for letter in &letters_non_zero {
            let c = (letter - b'A') as usize;
            banned_from_zero[c] = true;
        }
        let mut mapping = vec![0; A];
        let mut used_digit = vec![false; 10];
        go(
            0,
            &letters,
            &mut mapping,
            &banned_from_zero,
            &mut used_digit,
            &equation,
        )
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
    #[case(vec!["SEND".into(),"MORE".into()], "MONEY", true)]
    #[case(vec!["SIX".into(),"SEVEN".into(),"SEVEN".into()], "TWENTY", true)]
    #[case(vec!["LEET".into(),"CODE".into()], "POINT", false)]
    fn case(#[case] words: Vec<String>, #[case] result: String, #[case] expected: bool) {
        let actual = Solution::is_solvable(words, result);
        assert_eq!(actual, expected);
    }
}
