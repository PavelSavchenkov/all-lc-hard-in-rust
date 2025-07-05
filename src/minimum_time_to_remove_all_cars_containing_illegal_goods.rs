//! Solution for https://leetcode.com/problems/minimum-time-to-remove-all-cars-containing-illegal-goods
//! 2167. Minimum Time to Remove All Cars Containing Illegal Goods

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

impl Solution {
    pub fn minimum_time(s: String) -> i32 {
        let mut s = to_u8(&s);

        s.insert(0, b'1');
        s.push(b'1');

        let n = s.len();

        let mut a = Vec::new();
        for i in 0..n {
            if s[i] == b'1' {
                a.push(i as i32);
            }
        }

        let mut ans = i32::MAX;
        let mut min_right = i32::MAX;
        for i in (0..a.len()).rev() {
            if min_right < i32::MAX {
                ans = ans.min(1 + n as i32 - 2 + a[i] - 2 * i as i32 + min_right);
            }
            let func = -a[i] + 2 * i as i32;
            min_right = min_right.min(func);
        }

        ans - 2
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("1100101", 5)]
    #[case("0010", 2)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::minimum_time(s);
        assert_eq!(actual, expected);
    }
}
