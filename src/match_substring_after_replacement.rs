//! Solution for https://leetcode.com/problems/match-substring-after-replacement
//! 2301. Match Substring After Replacement

const A: usize = 26 + 26 + 10;

fn code_char(c: u8) -> usize {
    match c {
        b'a'..=b'z' => (c - b'a') as usize,
        b'A'..=b'Z' => 26 + (c - b'A') as usize,
        b'0'..=b'9' => 26 + 26 + (c - b'0') as usize,
        _ => panic!("c = {}", c as char),
    }
}

impl Solution {
    pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
        let s = to_u8(&s);
        let sub = to_u8(&sub);

        let mut can_map = vec![vec![false; A]; A];
        for map in &mappings {
            let from = code_char(map[0] as u8);
            let to = code_char(map[1] as u8);
            can_map[from][to] = true;
        }

        for start in 0..=s.len() - sub.len() {
            let mut failed = false;
            for i in 0..sub.len() {
                let from = sub[i];
                let to = s[start + i];
                if from == to {
                    continue;
                }
                let from = code_char(from);
                let to = code_char(to);
                if !can_map[from][to] {
                    failed = true;
                    break;
                }
            }
            if !failed {
                return true;
            }
        }

        false
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
    #[case("fool3e7bar", "leet", vec![vec!['e','3'],vec!['t','7'],vec!['t','8']], true)]
    #[case("fooleetbar", "f00l", vec![vec!['o','0']], false)]
    #[case("Fool33tbaR", "leetd", vec![vec!['e','3'],vec!['t','7'],vec!['t','8'],vec!['d','b'],vec!['p','b']], true)]
    fn case(
        #[case] s: String,
        #[case] sub: String,
        #[case] mappings: Vec<Vec<char>>,
        #[case] expected: bool,
    ) {
        let actual = Solution::match_replacement(s, sub, mappings);
        assert_eq!(actual, expected);
    }
}
