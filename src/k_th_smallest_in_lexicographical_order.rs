//! Solution for https://leetcode.com/problems/k-th-smallest-in-lexicographical-order
//! 440. K-th Smallest in Lexicographical Order

// how many numbers from [1; n] start with pref_digits
fn count(n_digits: &Vec<u8>, pref_digits: &Vec<u8>) -> usize {
    assert!(pref_digits.len() <= n_digits.len());

    let mut ans = 0;
    for len in pref_digits.len()..n_digits.len() {
        ans += usize::pow(10, (len - pref_digits.len()) as u32);
    }

    let mut is_equal = true;
    for i in 0..pref_digits.len() {
        if n_digits[i] != pref_digits[i] {
            is_equal = false;
            if n_digits[i] < pref_digits[i] {
                return ans;
            }
            break;
        }
    }
    if !is_equal {
        ans += usize::pow(10, (n_digits.len() - pref_digits.len()) as u32);
        return ans;
    }
    for pos_less in pref_digits.len()..n_digits.len() {
        let coef = n_digits[pos_less] as usize;
        ans += usize::pow(10, (n_digits.len() - 1 - pos_less) as u32) * coef;
    }
    ans += 1;
    ans
}

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut n = n as usize;
        let mut k = k as usize;

        let mut n_digits = Vec::new();
        while n > 0 {
            n_digits.push((n % 10) as u8);
            n /= 10;
        }
        n_digits.reverse();

        let mut ans = Vec::new();
        loop {
            // next digit
            for d in 0..10 {
                if d == 0 && ans.is_empty() {
                    continue;
                }
                ans.push(d as u8);
                let cur_count = count(&n_digits, &ans);
                if k <= cur_count {
                    break;
                }
                ans.pop();
                k -= cur_count;
            }
            if k == 1 {
                break;
            }
            k -= 1;
        }

        let mut num = 0;
        for &d in &ans {
            num = num * 10 + d as i32;
        }
        num
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(13, 2, 10)]
    #[case(1, 1, 1)]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::find_kth_number(n, k);
        assert_eq!(actual, expected);
    }
}
