//! Solution for https://leetcode.com/problems/parsing-a-boolean-expression
//! 1106. Parsing A Boolean Expression

fn parse(s: &Vec<u8>, i: usize) -> (bool, usize) {
    assert!(i < s.len());
    if s[i] == b'f' {
        return (false, i + 1);
    }
    if s[i] == b't' {
        return (true, i + 1);
    }
    if s[i] == b'!' {
        assert!(s[i + 1] == b'(');
        let (sub, end) = parse(s, i + 2);
        assert!(s[end] == b')');
        return (!sub, end + 1);
    }
    // & or |
    let op = s[i];
    let mut val = op == b'&';
    assert!(s[i + 1] == b'(');
    let mut i = i + 2;
    loop {
        let (sub, end) = parse(s, i);
        match op {
            b'&' => val &= sub,
            b'|' => val |= sub,
            _ => panic!("Wrong op = {}", op as char),
        }
        if s[end] != b')' {
            assert!(s[end] == b',');
            i = end + 1;
        } else {
            return (val, end + 1);
        }
    }
}

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let s = to_u8(&expression);

        let (val, end) = parse(&s, 0);
        assert!(end == s.len());
        val
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
    #[case("&(|(f))", false)]
    #[case("|(f,f,f,t)", true)]
    #[case("!(&(f,t))", true)]
    fn case(#[case] expression: String, #[case] expected: bool) {
        let actual = Solution::parse_bool_expr(expression);
        assert_eq!(actual, expected);
    }
}
