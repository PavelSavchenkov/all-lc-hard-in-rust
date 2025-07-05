//! Solution for https://leetcode.com/problems/largest-color-value-in-a-directed-graph
//! 1857. Largest Color Value in a Directed Graph

fn dfs(v: usize, g: &Vec<Vec<usize>>, color: &mut Vec<u32>, top: &mut Vec<usize>) -> bool {
    color[v] = 1;
    for &to in &g[v] {
        if color[to] == 1 {
            return false;
        }
        if color[to] == 0 {
            if !dfs(to, g, color, top) {
                return false;
            }
        }
    }
    color[v] = 2;
    top.push(v);
    true
}

const A: usize = 26;

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let colors = to_u8(&colors);
        let n = colors.len();

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
        }

        let mut color = vec![0; n];
        let mut top = Vec::new();
        for v in 0..n {
            if color[v] == 0 {
                if !dfs(v, &g, &mut color, &mut top) {
                    return -1;
                }
            }
        }

        let mut ans = 0;
        for c in 0..A {
            let mut dp = vec![0; n];
            for &v in &top {
                for &to in &g[v] {
                    dp[v] = dp[v].max(dp[to]);
                }
                if colors[v] == b'a' + c as u8 {
                    dp[v] += 1;
                }
                ans = ans.max(dp[v]);
            }
        }
        ans
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
    #[case("abaca", vec![vec![0,1],vec![0,2],vec![2,3],vec![3,4]], 3)]
    #[case("a", vec![vec![0,0]], -1)]
    fn case(#[case] colors: String, #[case] edges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::largest_path_value(colors, edges);
        assert_eq!(actual, expected);
    }
}
