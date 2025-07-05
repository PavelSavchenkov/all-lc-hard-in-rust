//! Solution for https://leetcode.com/problems/expression-add-operators
//! 282. Expression Add Operators

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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Token {
    Plus,
    Minus,
    Mul,
    Int(i64),
}

impl Token {
    fn from_u8(c: u8) -> Self {
        match c {
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Mul,
            b'0'..=b'9' => Token::Int((c - b'0') as i64),
            _ => panic!("This char is not a token: {}", c as char),
        }
    }

    fn do_op(&self, left: i64, right: i64) -> i64 {
        match self {
            Token::Plus => left + right,
            Token::Minus => left - right,
            Token::Mul => left * right,
            _ => panic!("Cannot do_op with non op token"),
        }
    }

    fn get_int(&self) -> Option<i64> {
        match self {
            Token::Int(num) => Some(*num),
            _ => None,
        }
    }

    fn is_plus_minus(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus)
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
        st.push(Token::Int(-num));
    } else {
        let num_left = st[st.len() - 1].get_int().unwrap();
        let res = op.do_op(num_left, num);
        *st.last_mut().expect("Last element should exist") = Token::Int(res);
    }
}

fn reduce_mul(st: &mut Vec<Token>) {
    assert!(!st.is_empty());
    if st.len() <= 2 {
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
    reduce_mul(st);
}

fn go(
    s: &Vec<u8>,
    i: usize,
    expr: &mut Vec<String>,
    st: &mut Vec<Token>,
    ans: &mut Vec<String>,
    target: i64,
) {
    if i == s.len() {
        let init_st = st.clone();
        let mut running_st = Vec::new();
        for t in st {
            running_st.push(*t);
            reduce_mul(&mut running_st);
        }
        reduce_sum(&mut running_st);
        assert!(running_st.len() == 1);
        if running_st[0] == Token::Int(target) {
            ans.push(expr.join(""));
            // eprintln!("init_st = {:#?}, expr = {:#?}", init_st, expr);
        }
        return;
    }

    let mut cur_num: i64 = 0;
    for j in i..s.len() {
        cur_num = cur_num * 10 + s[j] as i64;
        if s[i] == 0 && j > i {
            break;
        }

        expr.push(format!("{}", cur_num).to_string());
        st.push(Token::Int(cur_num));
        if j + 1 < s.len() {
            for op in b"+-*" {
                let op_token = Token::from_u8(*op);

                st.push(op_token);
                expr.push(format!("{}", *op as char));
                go(s, j + 1, expr, st, ans, target);
                expr.pop();
                st.pop();
            }
        } else {
            go(s, j + 1, expr, st, ans, target);
        }
        st.pop();
        expr.pop();
    }
}

impl Solution {
    pub fn add_operators(s: String, target: i32) -> Vec<String> {
        let s = to_u8(&s);
        let target = target as i64;

        let s: Vec<_> = s.iter().map(|&c| (c - b'0')).collect();

        let mut st = Vec::<Token>::new();
        let mut expr = Vec::<String>::new();
        let mut ans = Vec::<String>::new();
        go(&s, 0, &mut expr, &mut st, &mut ans, target);

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
    #[case("123", 6, vec!["1*2*3".into(),"1+2+3".into()])]
    #[case("232", 8, vec!["2*3+2".into(),"2+3*2".into()])]
    #[case("3456237490", 9191, vec![])]
    #[case("105", 5, vec!["1*0+5".into(),"10-5".into()])]
    fn case(#[case] num: String, #[case] target: i32, #[case] expected: Vec<String>) {
        let actual = Solution::add_operators(num, target);
        assert_eq!(actual, expected);
    }
}
