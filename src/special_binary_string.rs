//! Solution for https://leetcode.com/problems/special-binary-string
//! 761. Special Binary String

fn go(s: &Vec<u32>, mut i: usize) -> (Vec<u32>, usize) {
    assert!(s[i] == 1);

    if s[i + 1] == 0 {
        return (vec![1, 0], i + 2);
    }

    i += 1;
    let mut inside = Vec::new();
    loop {
        let (cur, end) = go(s, i);
        inside.push(cur);
        if s[end] == 0 {
            inside.sort();
            inside.reverse();
            let mut ans = vec![1];
            for item in &inside {
                ans.extend(item);
            }
            ans.push(0);
            return (ans, end + 1);
        }
        i = end;
    }
}

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut s: Vec<_> = s.as_bytes().iter().map(|&c| (c - b'0') as u32).collect();

        s.insert(0, 1);
        s.insert(s.len(), 0);
        let (mut ans, end) = go(&s, 0);
        assert!(end == s.len());

        ans.remove(0);
        ans.remove(ans.len() - 1);
        let ans = ans.iter().map(|&x| x as u8 + b'0').collect();
        from_u8(&ans)
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
    #[case("11011000", "11100100")]
    #[case("10", "10")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::make_largest_special(s);
        assert_eq!(actual, expected);
    }
}
