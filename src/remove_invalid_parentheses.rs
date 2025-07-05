//! Solution for https://leetcode.com/problems/remove-invalid-parentheses
//! 301. Remove Invalid Parentheses

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

fn go(
    s: &Vec<u8>,
    i: usize,
    cur_s: &mut Vec<u8>,
    bal: i32,
    min_removed: &mut usize,
    ans: &mut Vec<String>,
) {
    let removed = i - cur_s.len();
    if removed > *min_removed {
        return;
    }

    {
        let chars_after = s.len() - i;
        if bal > chars_after as i32 {
            return;
        }
    }

    if i == s.len() {
        if bal != 0 {
            return;
        }
        if removed < *min_removed {
            ans.clear();
            *min_removed = removed;
        }
        ans.push(from_u8(cur_s));
        return;
    }

    if s[i].is_ascii_alphabetic() {
        cur_s.push(s[i]);
        go(s, i + 1, cur_s, bal, min_removed, ans);
        cur_s.pop();
        return;
    }

    // remove
    go(s, i + 1, cur_s, bal, min_removed, ans);

    // keep
    let new_bal = if s[i] == b'(' { bal + 1 } else { bal - 1 };
    if new_bal < 0 {
        return;
    }
    cur_s.push(s[i]);
    go(s, i + 1, cur_s, new_bal, min_removed, ans);
    cur_s.pop();
}

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let s = to_u8(&s);

        let mut min_removed = usize::MAX;
        let mut ans = Vec::<String>::new();
        let bal = 0;
        let mut cur_s = Vec::<u8>::new();
        go(&s, 0, &mut cur_s, bal, &mut min_removed, &mut ans);

        ans.sort();
        ans.dedup();

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
    #[case("()())()", vec!["(())()".into(),"()()()".into()])]
    #[case("(a)())()", vec!["(a())()".into(),"(a)()()".into()])]
    #[case(")(", vec!["".into()])]
    fn case(#[case] s: String, #[case] expected: Vec<String>) {
        let actual = Solution::remove_invalid_parentheses(s);
        assert_eq!(actual, expected);
    }
}
