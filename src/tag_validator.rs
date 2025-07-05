//! Solution for https://leetcode.com/problems/tag-validator
//! 591. Tag Validator

fn get_match(s: &Vec<u8>, start: usize) -> Option<usize> {
    if start >= s.len() {
        return None;
    }
    if s[start] != b'<' {
        return None;
    }
    for i in start + 1..s.len() {
        if s[i] == b'>' {
            return Some(i);
        }
    }
    None
}

fn get_tag_name(s: &Vec<u8>, l: usize, r: usize) -> Option<Vec<u8>> {
    if l >= r || r > s.len() {
        return None;
    }
    let len = r - l;
    if !(1 <= len && len <= 9) {
        return None;
    }
    for i in l..r {
        if !s[i].is_ascii_uppercase() {
            return None;
        }
    }
    Some(s[l..r].to_vec())
}

fn get_start_tag(s: &Vec<u8>, start: usize) -> Option<(Vec<u8>, usize)> {
    let end = get_match(s, start);
    if end.is_none() {
        return None;
    }
    let end = end.unwrap();
    let name = get_tag_name(s, start + 1, end);
    if name.is_none() {
        return None;
    }
    let name = name.unwrap();
    Some((name, end + 1))
}

fn get_end_tag(s: &Vec<u8>, start: usize) -> Option<(Vec<u8>, usize)> {
    let end = get_match(s, start);
    if end.is_none() {
        return None;
    }
    if s[start + 1] != b'/' {
        return None;
    }
    let end = end.unwrap();
    let name = get_tag_name(s, start + 2, end);
    if name.is_none() {
        return None;
    }
    let name = name.unwrap();
    Some((name, end + 1))
}

fn get_cdata(s: &Vec<u8>, start: usize) -> Option<usize> {
    let pref = "<![CDATA[";
    if s.len() - start < pref.len() {
        return None;
    }
    if s[start..start + pref.len()].to_vec() != to_u8(&pref.to_string()) {
        return None;
    }

    // ]]>
    for i in start + pref.len()..=s.len() - 3 {
        if s[i] == b']' && s[i + 1] == b']' && s[i + 2] == b'>' {
            return Some(i + 3);
        }
    }

    None
}

fn get_tag_content(s: &Vec<u8>, start: usize) -> Option<usize> {
    if start >= s.len() {
        return Some(s.len());
    }

    if s[start] != b'<' {
        return get_tag_content(s, start + 1);
    }

    if start + 1 == s.len() {
        return None;
    }

    if s[start + 1] == b'/' {
        return Some(start);
    }
    if s[start + 1] == b'!' {
        let end = get_cdata(s, start);
        if end.is_none() {
            return None;
        }
        let end = end.unwrap();
        return get_tag_content(s, end);
    }

    let end = get_closed_tag(s, start);
    if end.is_none() {
        return None;
    }
    let end = end.unwrap();
    return get_tag_content(s, end);
}

fn get_closed_tag(s: &Vec<u8>, start: usize) -> Option<usize> {
    if start >= s.len() {
        return None;
    }

    if s.len() - start < 7 {
        return None;
    }

    let start_tag = get_start_tag(s, start);
    if start_tag.is_none() {
        return None;
    }
    let (name, end) = start_tag.unwrap();
    eprintln!("name={}, end={}", from_u8(&name), end);

    let end = get_tag_content(s, end);
    if end.is_none() {
        return None;
    }
    let end = end.unwrap();

    let end_tag = get_end_tag(s, end);
    if end_tag.is_none() {
        return None;
    }
    let (end_name, end) = end_tag.unwrap();
    if end_name != name {
        return None;
    }

    Some(end)
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s = to_u8(&s);

        let end = get_closed_tag(&s, 0);
        end == Some(s.len())
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
    #[case("<DIV>This is the first line <![CDATA[<div>]]></DIV>", true)]
    #[case("<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>", true)]
    #[case("<A>  <B> </A>   </B>", false)]
    fn case(#[case] code: String, #[case] expected: bool) {
        let actual = Solution::is_valid(code);
        assert_eq!(actual, expected);
    }
}
