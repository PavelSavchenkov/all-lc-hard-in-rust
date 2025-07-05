//! Solution for https://leetcode.com/problems/minimum-time-to-revert-word-to-initial-state-ii
//! 3031. Minimum Time to Revert Word to Initial State II

fn calc_z(s: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let mut l = 0;
    let mut r = 0;
    let mut z = vec![0; n];
    for i in 1..n {
        if i < r {
            z[i] = (r - i).min(z[i - l]);
        }
        while i + z[i] < n && s[i + z[i]] == s[z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z[0] = n;
    z
}

impl Solution {
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        let s = to_u8(&word);
        let k = k as usize;
        let n = s.len();

        let z = calc_z(&s);

        let mut ans = (n + k - 1) / k;
        for i in 1..n {
            if i + z[i] == n && i % k == 0 {
                ans = i / k;
                break;
            }
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
    #[case("abacaba", 3, 2)]
    #[case("abacaba", 4, 1)]
    #[case("abcbabcd", 2, 4)]
    fn case(#[case] word: String, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::minimum_time_to_initial_state(word, k);
        assert_eq!(actual, expected);
    }
}
