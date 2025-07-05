//! Solution for https://leetcode.com/problems/sum-of-k-mirror-numbers
//! 2081. Sum of k-Mirror Numbers

fn is_k_mirror(mut num: i64, k: usize) -> bool {
    assert!(num > 0);
    let k = k as i64;
    let mut digits = Vec::new();
    while num > 0 {
        let d = (num % k) as u8;
        digits.push(d);
        num /= k;
    }
    let mut digits_rev = digits.clone();
    digits_rev.reverse();
    digits == digits_rev
}

fn rec(i: usize, len: usize, k: usize, n: usize, pw10: &Vec<i64>, num: i64, ans: &mut Vec<i64>) {
    if ans.len() == n {
        return;
    }

    if i >= (len + 1) / 2 {
        if is_k_mirror(num, k) {
            ans.push(num);
        }
        return;
    }

    let d0 = if i == 0 { 1 } else { 0 };
    for d in d0..=9 {
        let mut new_num = num;
        new_num += pw10[len - 1 - i] * d as i64;
        if i < len - 1 - i {
            new_num += pw10[i] * d as i64;
        }
        rec(i + 1, len, k, n, pw10, new_num, ans);
    }
}

fn go(len: usize, k: usize, n: usize, ans: &mut Vec<i64>) {
    let pw10: Vec<_> = (0..len).map(|i| i64::pow(10, i as u32)).collect();

    rec(0, len, k, n, &pw10, 0, ans);
}

impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let n = 25;
        let k = 5;

        let k = k as usize;
        let n = n as usize;

        let mut ans = Vec::new();
        for len in 1..=16 {
            go(len, k, n, &mut ans);
        }

        // eprintln!("k={}, n={}, ans.len() = {}, ans={:#?}", k, n, ans.len(), ans);

        assert!(ans.len() == n);

        ans.iter().sum()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 5, 25)]
    #[case(3, 7, 499)]
    #[case(7, 17, 20379000)]
    #[case(5, 15, 20526195)]
    fn case(#[case] k: i32, #[case] n: i32, #[case] expected: i64) {
        let actual = Solution::k_mirror(k, n);
        assert_eq!(actual, expected);
    }
}
