//! Solution for https://leetcode.com/problems/number-of-digit-one
//! 233. Number of Digit One

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut n = n as usize;
        n += 1;

        let mut a = Vec::new();
        while n > 0 {
            a.push((n % 10) as u8);
            n /= 10;
        }
        a.reverse();

        let mut pw10 = vec![1; a.len() + 1];
        for i in 1..pw10.len() {
            pw10[i] = pw10[i - 1] * 10;
        }

        let mut ans: u32 = 0;
        for pos1 in 0..a.len() {
            for pos_less in 0..a.len() {
                if pos_less < pos1 {
                    let mut cnt = 1;
                    cnt *= a[pos_less] as u32;
                    cnt *= pw10[a.len() - pos_less - 1 - 1];
                    ans += cnt;
                } else if pos_less == pos1 {
                    if a[pos_less] <= 1 {
                        continue;
                    }
                    let cnt = pw10[a.len() - pos1 - 1];
                    ans += cnt;
                } else {
                    assert!(pos_less > pos1);
                    if a[pos1] != 1 {
                        continue;
                    }
                    let mut cnt = 1;
                    cnt *= a[pos_less] as u32;
                    cnt *= pw10[a.len() - pos_less - 1];
                    ans += cnt;
                }
            }
        }

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
    #[case(13, 6)]
    #[case(0, 0)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::count_digit_one(n);
        assert_eq!(actual, expected);
    }
}
