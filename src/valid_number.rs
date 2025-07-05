//! Solution for https://leetcode.com/problems/valid-number
//! 65. Valid Number

fn read_digits(s: &Vec<u8>, i: usize) -> Option<usize> {
    if i >= s.len() {
        return None;
    }
    for ii in i..s.len() {
        if !s[ii].is_ascii_digit() {
            if ii == i {
                return None;
            }
            return Some(ii);
        }
    }
    Some(s.len())
}

fn read_integer(s: &Vec<u8>, mut i: usize) -> Option<usize> {
    if i >= s.len() {
        return None;
    }
    if matches!(s[i], b'+' | b'-') {
        i += 1;
    }
    read_digits(s, i)
}

fn read_decimal(s: &Vec<u8>, mut i: usize) -> Option<usize> {
    if i >= s.len() {
        return None;
    }
    if matches!(s[i], b'+' | b'-') {
        i += 1;
        if i == s.len() {
            return None;
        }
    }
    if s[i] == b'.' {
        i += 1;
        return read_digits(s, i);
    }
    let i = read_digits(s, i);
    if i.is_none() {
        return None;
    }
    let i = i.unwrap();
    if i == s.len() {
        return None;
    }
    if s[i] != b'.' {
        return None;
    }
    let i_digs = read_digits(s, i + 1);
    if i_digs.is_none() {
        return Some(i + 1);
    } else {
        return i_digs;
    }
}

fn read_exponent(s: &Vec<u8>, mut i: usize) -> Option<usize> {
    if i >= s.len() {
        return None;
    }
    if s[i].to_ascii_lowercase() != b'e' {
        return None;
    }
    i += 1;
    read_integer(s, i)
}

fn read_valid(s: &Vec<u8>, i: usize) -> Option<usize> {
    for it in 0..2 {
        let ii = if it == 0 {
            read_integer(s, i)
        } else {
            read_decimal(s, i)
        };
        if !ii.is_none() {
            let ii = ii.unwrap();
            if ii == s.len() {
                return Some(s.len());
            }
            let ii = read_exponent(s, ii);
            if ii == Some(s.len()) {
                return ii;
            }
        }
    }
    None
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let s: Vec<_> = s.as_bytes().iter().cloned().collect();
        read_valid(&s, 0) == Some(s.len())
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("0", true)]
    #[case("e", false)]
    #[case(".", false)]
    fn case(#[case] s: String, #[case] expected: bool) {
        let actual = Solution::is_number(s);
        assert_eq!(actual, expected);
    }
}
