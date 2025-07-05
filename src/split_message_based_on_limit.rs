//! Solution for https://leetcode.com/problems/split-message-based-on-limit
//! 2468. Split Message Based on Limit

impl Solution {
    pub fn split_message(s: String, limit: i32) -> Vec<String> {
        let limit = limit as usize;
        let s = to_u8(&s);

        let mut max_len_b: usize = 1;
        while usize::pow(10, max_len_b as u32) < s.len() {
            max_len_b += 1;
        }

        for len_b in 1..=max_len_b {
            let mut i = 1;
            let mut used_from_s = 0;
            let mut ans = Vec::new();
            while used_from_s < s.len() {
                let i_str = i.to_string();
                let suff_len = i_str.len() + 3 + len_b;
                if suff_len >= limit {
                    break;
                }
                let pref_len = (limit - suff_len).min(s.len() - used_from_s);
                let mut part = Vec::new();
                part.extend_from_slice(&s[used_from_s..used_from_s + pref_len]);
                part.extend_from_slice(&"<".as_bytes());
                part.extend_from_slice(&i_str.as_bytes());
                part.extend_from_slice(&"/".as_bytes());
                ans.push(part);
                used_from_s += pref_len;
                i += 1
            }
            if used_from_s != s.len() {
                continue;
            }
            let cnt_parts = ans.len();
            let b = to_u8(&cnt_parts.to_string());
            let actual_len_b = b.len();
            if actual_len_b != len_b {
                continue;
            }
            for part in &mut ans {
                part.extend_from_slice(&b);
                part.extend_from_slice(&">".as_bytes());
            }
            return from_u8_vec(&ans);
        }
        Vec::new()
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
    #[case("this is really a very awesome message", 9, vec!["thi&lt;1/14&gt;".into(),"s i&lt;2/14&gt;".into(),"s r&lt;3/14&gt;".into(),"eal&lt;4/14&gt;".into(),"ly &lt;5/14&gt;".into(),"a v&lt;6/14&gt;".into(),"ery&lt;7/14&gt;".into()," aw&lt;8/14&gt;".into(),"eso&lt;9/14&gt;".into(),"me&lt;10/14&gt;".into()," m&lt;11/14&gt;".into(),"es&lt;12/14&gt;".into(),"sa&lt;13/14&gt;".into(),"ge&lt;14/14&gt;".into()])]
    #[case("short message", 15, vec!["short mess&lt;1/2&gt;".into(),"age&lt;2/2&gt;".into()])]
    fn case(#[case] message: String, #[case] limit: i32, #[case] expected: Vec<String>) {
        let actual = Solution::split_message(message, limit);
        assert_eq!(actual, expected);
    }
}
