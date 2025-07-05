//! Solution for https://leetcode.com/problems/largest-palindrome-product
//! 479. Largest Palindrome Product

fn is_prod(x: u64, n: usize) -> bool {
    // eprintln!("check x={}, n={}", x, n);
    let low = u64::pow(10, (n - 1) as u32);
    let up = u64::pow(10, n as u32);
    for d in (low..up).rev() {
        if x % d == 0 {
            let dd = x / d;
            if low <= dd && dd < up {
                return true;
            }
        }
        if d * d < x {
            break
        }
    }
    false
}

fn rec(len: usize, pos: usize, pw10: &Vec<u64>, cur_num: u64, ans: &mut u64) {
    if *ans != 0 {
        return;
    }

    if pos == (len + 1) / 2 {
        if is_prod(cur_num, (len + 1) / 2) {
            *ans = cur_num;
        }
        return;
    }

    for d in (0..=9).rev() {
        if pos == 0 && d == 0 {
            continue;
        }
        let mut new_num = cur_num;
        new_num += d as u64 * pw10[pos];
        if len - pos - 1 > pos {
            new_num += d as u64 * pw10[len - pos - 1];
        }
        rec(len, pos + 1, pw10, new_num, ans);
    }
}

fn solve(n: i32) -> u64 {
    let n = n as usize;

    let mut pw10 = vec![1 as u64; n * 2];
    for i in 1..pw10.len() {
        pw10[i] = pw10[i - 1] * 10 as u64;
    }

    let cur_num: u64 = 0;
    let mut ans: u64 = 0;
    rec(2 * n, 0, &pw10, cur_num, &mut ans);
    rec(2 * n - 1, 0, &pw10, cur_num, &mut ans);

    ans
}

impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        (solve(n) % 1337) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 987)]
    // #[case(1, 9)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::largest_palindrome(n);
        assert_eq!(actual, expected);
    }
}
