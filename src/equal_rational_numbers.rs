//! Solution for https://leetcode.com/problems/equal-rational-numbers
//! 972. Equal Rational Numbers

fn add_to_vec_u8(s: &mut Vec<u8>, val: u8) -> u8 {
    let mut carry = val;
    for i in (0..s.len()).rev() {
        let dig = s[i] - b'0';
        let new_dig = dig + carry;
        if new_dig < 10 {
            s[i] = b'0' + new_dig;
            carry = 0;
            break;
        } else {
            s[i] = b'0' + (new_dig % 10);
            carry = new_dig / 10;
        }
    }
    carry
}

#[derive(Eq, PartialEq)]
struct Number {
    integer: Vec<u8>,
    non_repeating: Vec<u8>,
    repeating: Vec<u8>,
}

impl Number {
    fn new(s: &Vec<u8>) -> Self {
        let mut integer = Vec::new();
        let mut non_repeating = Vec::new();
        let mut repeating = Vec::new();
        let mut i = 0;
        while i < s.len() && s[i] != b'.' {
            integer.push(s[i]);
            i += 1;
        }
        if i < s.len() {
            assert!(s[i] == b'.');
            i += 1;
            while i < s.len() && s[i] != b'(' {
                non_repeating.push(s[i]);
                i += 1;
            }
            if i < s.len() {
                assert!(s[i] == b'(');
                i += 1;
                while s[i] != b')' {
                    repeating.push(s[i]);
                    i += 1;
                }
            }
        }

        let is_repeating_all_9 =
            !repeating.is_empty() && repeating.iter().filter(|&&c| c != b'9').next().is_none();
        if is_repeating_all_9 {
            repeating.clear();
            let carry = add_to_vec_u8(&mut non_repeating, 1);
            if carry != 0 {
                let carry = add_to_vec_u8(&mut integer, carry);
                if carry != 0 {
                    assert!(carry < 10);
                    let forward_dig = b'0' + carry;
                    integer.insert(0, forward_dig);
                }
            }
        }

        if repeating.is_empty() {
            repeating.push(b'0');
        }

        Self {
            integer,
            non_repeating,
            repeating,
        }
    }

    fn norm(&mut self, non_repeating_len: usize, repeating_len: usize) {
        while self.non_repeating.len() < non_repeating_len {
            if self.repeating.is_empty() {
                self.non_repeating.push(b'0');
            } else {
                self.non_repeating.push(self.repeating[0]);
                self.repeating.rotate_left(1);
            }
        }
        assert!(repeating_len % self.repeating.len() == 0);
        let times = repeating_len / self.repeating.len();
        self.repeating = self
            .repeating
            .iter()
            .copied()
            .cycle()
            .take(repeating_len)
            .collect();
        assert!(self.non_repeating.len() == non_repeating_len);
        assert!(self.repeating.len() == repeating_len);
    }

    fn to_string(&self) -> String {
        let mut str = String::new();
        str.push_str(&from_u8(&self.integer));
        str.push('.');
        str.push_str(&from_u8(&self.non_repeating));
        if !self.repeating.is_empty() {
            str.push('(');
            str.push_str(&from_u8(&self.repeating));
            str.push(')');
        }
        str
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a / gcd(a, b)) * b
}

impl Solution {
    pub fn is_rational_equal(s: String, t: String) -> bool {
        let s = to_u8(&s);
        let t = to_u8(&t);

        let mut s = Number::new(&s);
        let mut t = Number::new(&t);

        eprintln!("s = {}, t = {}", s.to_string(), t.to_string());

        let non_repeating_len = s.non_repeating.len().max(t.non_repeating.len());
        let repeating_len = lcm(s.repeating.len(), t.repeating.len());
        s.norm(non_repeating_len, repeating_len);
        t.norm(non_repeating_len, repeating_len);
        s == t
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
    #[case("0.(52)", "0.5(25)", true)]
    #[case("0.1666(6)", "0.166(66)", true)]
    #[case("0.9(9)", "1.", true)]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: bool) {
        let actual = Solution::is_rational_equal(s, t);
        assert_eq!(actual, expected);
    }
}
