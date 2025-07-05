//! Solution for https://leetcode.com/problems/smallest-good-base
//! 483. Smallest Good Base

fn get_ones_in_this_base(base: u64, len: usize) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..len {
        let cur = base.saturating_pow(i as u32);
        sum = sum.saturating_add(cur);
    }
    sum
}

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let n: u64 = n.parse().expect("Should be valid number");

        for len in (1..64).rev() {
            let mut L: u64 = 1;
            let mut R: u64 = n;
            while L + 1 != R {
                let M = (L + R) / 2;
                let sum = get_ones_in_this_base(M, len);
                if sum < n {
                    L = M
                } else {
                    R = M
                }
            }
            let base = R;
            if get_ones_in_this_base(base, len) == n {
                return base.to_string();
            }
        }
        unreachable!()
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
    #[case("13", "3")]
    #[case("4681", "8")]
    #[case("1000000000000000000", "999999999999999999")]
    fn case(#[case] n: String, #[case] expected: String) {
        let actual = Solution::smallest_good_base(n);
        assert_eq!(actual, expected);
    }
}
