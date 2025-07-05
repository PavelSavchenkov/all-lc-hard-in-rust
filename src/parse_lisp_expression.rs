//! Solution for https://leetcode.com/problems/parse-lisp-expression
//! 736. Parse Lisp Expression

use std::collections::HashMap;

#[derive(Default)]
struct Scope {
    vars: HashMap<u64, i32>,
}

impl Scope {
    fn new() -> Self {
        Self::default()
    }

    fn get_val(&self, var: u64) -> Option<i32> {
        self.vars.get(&var).copied()
    }
}

fn get_int(s: &Vec<u8>, mut i: usize) -> (i32, usize) {
    let mut coef = 1;
    if s[i] == b'-' {
        coef = -1;
        i += 1;
    }
    let mut num = 0;
    while i < s.len() && s[i].is_ascii_digit() {
        num = num * 10 + (s[i] - b'0') as i32;
        i += 1;
    }
    (num * coef, i)
}

fn get_var(s: &Vec<u8>, mut i: usize) -> (u64, usize) {
    let mut var = Vec::new();
    while i < s.len() && (s[i].is_ascii_digit() || s[i].is_ascii_lowercase()) {
        var.push(s[i]);
        i += 1;
    }
    (get_hash(&var), i)
}

fn parse(s: &Vec<u8>, mut i: usize, st: &mut Vec<Scope>) -> (i32, usize) {
    assert!(i < s.len());

    // eprintln!("parse {}", from_u8(&s[i..].to_vec()));

    if s[i] != b'(' {
        if s[i].is_ascii_lowercase() {
            let (var, end) = get_var(s, i);
            for j in (0..st.len()).rev() {
                if let Some(num) = st[j].get_val(var) {
                    return (num, end);
                }
            }
            unreachable!()
        } else {
            return get_int(s, i);
        }
    }

    i += 1;
    // let
    if s[i] == b'l' {
        i += 4;
        st.push(Scope::new());
        loop {
            let mut is_final_expr = false;
            if s[i] == b'(' || !s[i].is_ascii_lowercase() {
                is_final_expr = true;
            } else {
                let (var, end) = get_var(s, i);
                if s[end] == b')' {
                    is_final_expr = true;
                }
            }

            if is_final_expr {
                let (num, end) = parse(s, i, st);
                assert!(s[end] == b')');
                st.pop();
                return (num, end + 1);
            }

            let (var, end) = get_var(s, i);
            assert!(s[end] == b' ');
            let (e, end) = parse(s, end + 1, st);
            i = end + 1;
            st.last_mut().unwrap().vars.insert(var, e);
        }
    }

    // add or mult
    // TODO: enum for operation
    let mut is_add = false;
    if s[i] == b'a' {
        i += 4;
        is_add = true;
    } else {
        assert!(s[i] == b'm');
        i += 5;
    }

    let (e1, end) = parse(s, i, st);
    let (e2, end) = parse(s, end + 1, st);
    let num = if is_add { e1 + e2 } else { e1 * e2 };
    assert!(s[end] == b')');
    (num, end + 1)
}

impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        let s = to_u8(&expression);

        let mut st = Vec::new();
        let (num, end) = parse(&s, 0, &mut st);
        assert!(end == s.len());
        num
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

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("(let x 2 (mult x (let x 3 y 4 (add x y))))", 14)]
    #[case("(let x 3 x 2 x)", 2)]
    #[case("(let x 1 y 2 x (add x y) (add x y))", 5)]
    fn case(#[case] expression: String, #[case] expected: i32) {
        let actual = Solution::evaluate(expression);
        assert_eq!(actual, expected);
    }
}
