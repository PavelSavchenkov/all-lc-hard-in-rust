//! Solution for https://leetcode.com/problems/find-substring-with-given-hash-value
//! 2156. Find Substring With Given Hash Value

fn mul(a: i32, b: i32, modulo: i32) -> i32 {
    let res = (a as i64 * b as i64) % modulo as i64;
    res as i32
}

fn add(a: i32, b: i32, modulo: i32) -> i32 {
    let mut res = a + b;
    if res >= modulo {
        res -= modulo
    }
    res
}

fn sub(a: i32, b: i32, modulo: i32) -> i32 {
    let mut res = a - b;
    if res < 0 {
        res += modulo
    }
    res
}

impl Solution {
    pub fn sub_str_hash(s: String, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
        let initial_s = s.clone();
        let s: Vec<_> = s
            .as_bytes()
            .iter()
            .map(|&b| ((b - b'a' + 1) as i32) % modulo)
            .collect();
        let n = s.len();
        let k = k as usize;

        let mut pw = vec![1; n + 1];
        for i in 1..=n {
            pw[i] = mul(pw[i - 1], power, modulo);
        }

        let mut h = 0;
        for i in n - k..n {
            let cur = mul(s[i], pw[i - (n - k)], modulo);
            h = add(h, cur, modulo);
        }

        let mut ans = n;
        for i in (0..=n - k).rev() {
            if h == hash_value {
                ans = i;
            }
            if i == 0 {
                break;
            }
            h = mul(h, power, modulo);
            let last = mul(s[i + k - 1], pw[k], modulo);
            h = sub(h, last, modulo);
            let first = s[i - 1];
            h = add(h, first, modulo);
        }
        assert!(ans < n);

        initial_s[ans..ans + k].to_string()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("leetcode", 7, 20, 2, 0, "ee")]
    #[case("fbxzaad", 31, 100, 3, 32, "fbx")]
    fn case(
        #[case] s: String,
        #[case] power: i32,
        #[case] modulo: i32,
        #[case] k: i32,
        #[case] hash_value: i32,
        #[case] expected: String,
    ) {
        let actual = Solution::sub_str_hash(s, power, modulo, k, hash_value);
        assert_eq!(actual, expected);
    }
}
