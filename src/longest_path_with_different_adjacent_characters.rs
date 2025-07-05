//! Solution for https://leetcode.com/problems/longest-path-with-different-adjacent-characters
//! 2246. Longest Path With Different Adjacent Characters

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

fn dfs(v: usize, g: &Vec<Vec<usize>>, s: &Vec<u8>, ans: &mut usize) -> usize {
    let mut mx1 = 0;
    let mut mx2 = 0;
    for &to in &g[v] {
        let down = dfs(to, g, s, ans);
        if s[to] == s[v] {
            continue;
        }
        if down > mx1 {
            mx2 = mx1;
            mx1 = down;
        } else if down > mx2 {
            mx2 = down;
        }
    }

    *ans = (*ans).max(1 + mx1 + mx2);

    mx1 + 1
}

impl Solution {
    pub fn longest_path(par: Vec<i32>, s: String) -> i32 {
        let n = par.len();
        let s = to_u8(&s);

        let mut g = vec![Vec::new(); n];
        for i in 1..n {
            let p = par[i] as usize;
            g[p].push(i);
        }

        let mut ans = 0;
        dfs(0, &g, &s, &mut ans);

        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![-1,0,0,1,1,2], "abacbe", 3)]
    #[case(vec![-1,0,0,0], "aabc", 3)]
    fn case(#[case] parent: Vec<i32>, #[case] s: String, #[case] expected: i32) {
        let actual = Solution::longest_path(parent, s);
        assert_eq!(actual, expected);
    }
}
