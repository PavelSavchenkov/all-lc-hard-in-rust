//! Solution for https://leetcode.com/problems/minimum-cost-to-convert-string-ii
//! 2977. Minimum Cost to Convert String II

fn calc_prefix_function(s: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let mut p = vec![0; n];
    for i in 1..n {
        let mut j = p[i - 1];
        while j > 0 && s[i] != s[j] {
            j = p[j - 1];
        }
        if s[i] == s[j] {
            p[i] = j + 1;
        }
    }
    p
}

fn calc_is_sub(pattern: &Vec<u8>, s: &Vec<u8>) -> Vec<bool> {
    let mut str = pattern.clone();
    str.push(0);
    str.extend(s.iter());
    let p = calc_prefix_function(&str);
    let mut is_sub = vec![false; s.len()];
    for i in pattern.len()..str.len() {
        if p[i] == pattern.len() && i + 1 >= pattern.len() + pattern.len() + 1 {
            is_sub[i - 2 * pattern.len()] = true;
        }
    }
    is_sub
}

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        let source = to_u8(&source);
        let target = to_u8(&target);
        let original = to_u8_vec(&original);
        let changed = to_u8_vec(&changed);

        let n = source.len();
        assert!(target.len() == n);

        let n_changes = original.len();
        assert!(changed.len() == n_changes);

        let mut subs = Vec::new();
        for i in 0..n_changes {
            subs.push(original[i].clone());
            subs.push(changed[i].clone());
        }
        subs.sort();
        subs.dedup();

        let mut g = vec![vec![i32::MAX; subs.len()]; subs.len()];
        for i in 0..subs.len() {
            g[i][i] = 0;
        }
        for i in 0..n_changes {
            let from = subs.binary_search(&original[i]).unwrap();
            let to = subs.binary_search(&changed[i]).unwrap();
            g[from][to] = g[from][to].min(cost[i]);
        }

        for k in 0..subs.len() {
            for i in 0..subs.len() {
                for j in 0..subs.len() {
                    let left = g[i][k];
                    let right = g[k][j];
                    if left < i32::MAX && right < i32::MAX {
                        g[i][j] = g[i][j].min(left + right);
                    }
                }
            }
        }

        let mut is_sub_source = vec![Vec::new(); subs.len()];
        let mut is_sub_target = vec![Vec::new(); subs.len()];
        for j in 0..subs.len() {
            is_sub_source[j] = calc_is_sub(&subs[j], &source);
            is_sub_target[j] = calc_is_sub(&subs[j], &target);
        }
        let mut trans = vec![vec![i64::MAX; n + 1]; n];
        for i in 0..n {
            for a in 0..subs.len() {
                if !is_sub_source[a][i] {
                    continue;
                }
                let len = subs[a].len();
                for b in 0..subs.len() {
                    if g[a][b] == i32::MAX {
                        continue;
                    }
                    if !is_sub_target[b][i] {
                        continue;
                    }
                    assert!(subs[b].len() == len);
                    trans[i][i + len] = trans[i][i + len].min(g[a][b] as i64);
                }
            }
        }
        for i in 0..n {
            if source[i] == target[i] {
                trans[i][i + 1] = 0;
            }
        }

        let mut dp = vec![i64::MAX; n + 1];
        dp[0] = 0;
        for i in 0..n {
            let cur_dp = dp[i];
            if cur_dp == i64::MAX {
                continue;
            }
            for j in i + 1..=n {
                let t = trans[i][j];
                if t < i64::MAX {
                    dp[j] = dp[j].min(cur_dp + t);
                }
            }
        }
        let ans = dp[n];
        if ans == i64::MAX {
            return -1;
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
    #[case("abcd", "acbe", vec!["a".into(),"b".into(),"c".into(),"c".into(),"e".into(),"d".into()], vec!["b".into(),"c".into(),"b".into(),"e".into(),"b".into(),"e".into()], vec![2,5,5,1,2,20], 28)]
    #[case("abcdefgh", "acdeeghh", vec!["bcd".into(),"fgh".into(),"thh".into()], vec!["cde".into(),"thh".into(),"ghh".into()], vec![1,3,5], 9)]
    #[case("abcdefgh", "addddddd", vec!["bcd".into(),"defgh".into()], vec!["ddd".into(),"ddddd".into()], vec![100,1578], -1)]
    #[case("aaabbebbbhbbbbebaaeh", "hhbebebbahhhehhbbhee", vec!["a".into(),"b".into(), "b".into(), "b".into(), "e".into(), "a".into(), "h".into()], vec!["b".into(),"e".into(), "a".into(), "h".into(), "h".into(), "h".into(), "e".into()], vec![9,8,5,9,3,7,9], 99)]
    fn case(
        #[case] source: String,
        #[case] target: String,
        #[case] original: Vec<String>,
        #[case] changed: Vec<String>,
        #[case] cost: Vec<i32>,
        #[case] expected: i64,
    ) {
        let actual = Solution::minimum_cost(source, target, original, changed, cost);
        assert_eq!(actual, expected);
    }
}
