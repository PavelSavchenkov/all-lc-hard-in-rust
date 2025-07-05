//! Solution for https://leetcode.com/problems/basic-calculator
//! 224. Basic Calculator

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

#[derive(PartialEq, Copy, Clone)]
enum Token {
    OpenBracket,
    CloseBracket,
    Plus,
    Minus,
    Mul,
    Int(i32),
}

impl Token {
    fn from_u8(c: u8) -> Self {
        match c {
            b'(' => Token::OpenBracket,
            b')' => Token::CloseBracket,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Mul,
            _ => panic!("Wrong char for token {}", c as char),
        }
    }

    fn from_int(num: i32) -> Self {
        Token::Int(num)
    }

    fn is_plus_minus(&self) -> bool {
        match self {
            Token::Plus | Token::Minus => true,
            _ => false,
        }
    }

    fn do_op(&self, left: i32, right: i32) -> i32 {
        match self {
            Token::Plus => left + right,
            Token::Minus => left - right,
            Token::Mul => left * right,
            _ => panic!("Wrong token to do op"),
        }
    }

    fn get_int(&self) -> Option<i32> {
        match self {
            Token::Int(num) => Some(*num),
            _ => None,
        }
    }
}

fn reduce_sum(st: &mut Vec<Token>) {
    if st.len() < 2 {
        return;
    }

    let num = st[st.len() - 1].get_int();
    if num.is_none() {
        return;
    }
    let num = num.unwrap();
    if !st[st.len() - 2].is_plus_minus() {
        return;
    }
    let op = st[st.len() - 2];
    st.truncate(st.len() - 2);
    reduce_sum(st);
    if st.is_empty() || st[st.len() - 1].get_int().is_none() {
        assert!(op == Token::Minus);
        st.push(Token::from_int(-num));
    } else {
        let num_left = st[st.len() - 1].get_int().unwrap();
        let res = op.do_op(num_left, num);
        *st.last_mut().expect("Last element should exist") = Token::Int(res);
    }
}

fn reduce(st: &mut Vec<Token>) {
    assert!(!st.is_empty());
    if st.len() <= 2 {
        return;
    }

    if st[st.len() - 1] == Token::CloseBracket {
        st.pop();
        reduce_sum(st);
        assert!(st[st.len() - 2] == Token::OpenBracket);
        st.swap_remove(st.len() - 2);
        reduce(st);
        return;
    }

    let num = st[st.len() - 1].get_int();
    if num.is_none() {
        return;
    }
    let num = num.unwrap();
    if st[st.len() - 2] != Token::Mul {
        return;
    }
    let num_left = st[st.len() - 3].get_int().unwrap();
    let res = num_left * num;
    st.truncate(st.len() - 3);
    st.push(Token::Int(res));
    reduce(st);
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = to_u8(&s);

        let mut st = Vec::new();
        let mut i = 0;
        while i < s.len() {
            if s[i] == b' ' {
                i += 1;
                continue;
            }
            if !s[i].is_ascii_digit() {
                st.push(Token::from_u8(s[i]));
                i += 1;
            } else {
                let mut num = 0;
                let mut new_i = 0;
                for j in i..s.len() {
                    if !s[j].is_ascii_digit() {
                        break;
                    }
                    num = num * 10 + (s[j] - b'0') as i32;
                    new_i = j + 1;
                }
                i = new_i;
                st.push(Token::from_int(num));
            }
            reduce(&mut st);
        }
        reduce_sum(&mut st);

        assert!(st.len() == 1);
        st[0].get_int().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("1 + 1", 2)]
    #[case(" 2-1 + 2 ", 3)]
    #[case("(1+(4+5+2)-3)+(6+8)", 23)]
    #[case("1-(-2)", 3)]
    #[case("-2+1", -1)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::calculate(s);
        assert_eq!(actual, expected);
    }
}
