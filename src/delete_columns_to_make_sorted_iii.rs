//! Solution for https://leetcode.com/problems/delete-columns-to-make-sorted-iii
//! 960. Delete Columns to Make Sorted III

fn remin(a: &mut usize, b: usize) {
    if *a > b {
        *a = b;
    }
}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strs = to_u8_vec(&strs);

        let n = strs.len();
        let len = strs[0].len();
        let mut dp = vec![usize::MAX; len];
        for i in 0..len {
            dp[i] = i;
            for j in 0..i {
                let mut ok = true;
                for k in 0..n {
                    if strs[k][j] > strs[k][i] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }
                let prev_dp = dp[j];
                remin(&mut dp[i], prev_dp + i - j - 1);
            }
        }
        let mut ans = usize::MAX;
        for i in 0..len {
            let cur = dp[i] + len - i - 1;
            remin(&mut ans, cur);
        }
        ans as i32
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
    #[case(vec!["babca".into(),"bbazb".into()], 3)]
    #[case(vec!["edcba".into()], 4)]
    #[case(vec!["ghi".into(),"def".into(),"abc".into()], 0)]
    fn case(#[case] strs: Vec<String>, #[case] expected: i32) {
        let actual = Solution::min_deletion_size(strs);
        assert_eq!(actual, expected);
    }
}
