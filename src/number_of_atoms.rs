//! Solution for https://leetcode.com/problems/number-of-atoms
//! 726. Number of Atoms

use std::collections::HashMap;

#[derive(Default)]
struct Ans {
    count: HashMap<Vec<u8>, u32>,
}

impl Ans {
    fn new() -> Self {
        Self::default()
    }

    fn merge_from(&mut self, other: &Self) {
        for (name, cnt) in &other.count {
            let key = name.clone();
            *self.count.entry(key).or_insert(0) += cnt;
        }
    }

    fn multiply(&mut self, coef: u32) {
        if coef == 1 {
            return;
        }
        for (name, cnt) in &mut self.count {
            *cnt *= coef;
        }
    }
}

fn get_count(s: &Vec<u8>, i: usize) -> (u32, usize) {
    let mut j = i;
    let mut count = 0;
    while j < s.len() && s[j].is_ascii_digit() {
        count = count * 10 + (s[j] - b'0') as u32;
        j += 1;
    }
    (count.max(1), j)
}

fn get_ans(s: &Vec<u8>, i: usize) -> (Ans, usize) {
    if i == s.len() || s[i] == b')' {
        return (Ans::new(), i);
    }

    // eprintln!("get_ans from {}", from_u8(&s[i..].to_vec()));

    let mut ans = Ans::new();
    let mut end = 0;
    if s[i] == b'(' {
        (ans, end) = get_ans(s, i + 1);
        assert!(s[end] == b')');
        end += 1;
    } else {
        assert!(s[i].is_ascii_uppercase());
        let mut j = i + 1;
        while j < s.len() && s[j].is_ascii_lowercase() {
            j += 1;
        }
        let name = s[i..j].to_vec();
        ans.count.insert(name, 1);
        end = j;
    }
    assert!(end > 0);

    let (count, end) = get_count(s, end);
    ans.multiply(count);

    let (suff, end) = get_ans(s, end);
    ans.merge_from(&suff);
    (ans, end)
}

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let s = to_u8(&formula);

        let (ans, end) = get_ans(&s, 0);
        assert!(end == s.len());

        let mut items = Vec::new();
        for (name, cnt) in &ans.count {
            items.push((name.clone(), *cnt));
        }
        items.sort();

        let mut str = String::new();
        for (name, cnt) in &items {
            let mut s = from_u8(&name);
            if *cnt > 1 {
                s.push_str(&cnt.to_string());
            }
            str.push_str(&s);
        }

        str
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
    #[case("H2O", "H2O")]
    #[case("Mg(OH)2", "H2MgO2")]
    #[case("K4(ON(SO3)2)2", "K4N2O14S4")]
    fn case(#[case] formula: String, #[case] expected: String) {
        let actual = Solution::count_of_atoms(formula);
        assert_eq!(actual, expected);
    }
}
