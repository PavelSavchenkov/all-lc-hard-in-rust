//! Solution for https://leetcode.com/problems/minimum-number-of-days-to-eat-n-oranges
//! 1553. Minimum Number of Days to Eat N Oranges

fn lower_bound(n: usize) -> usize {
    let mut ans = 0;
    let mut nn = n;
    while nn > 0 {
        nn /= 3;
        ans += 1;
    }
    ans
}

fn go(n: usize, days: usize, ans: &mut usize) -> usize {
    if n == 0 {
        *ans = (*ans).min(days);
        return 0;
    }

    if days + lower_bound(n) >= *ans {
        return usize::MAX;
    }

    let mut best = usize::MAX;
    if n % 2 == 0 {
        best = best.min(go(n / 2, days + 1, ans));
    }
    if n % 3 == 0 {
        best = best.min(go(n / 3, days + 1, ans));
    }
    best = best.min(go(n - 1, days + 1, ans));
    if best < usize::MAX {
        best += 1;
    }

    best
}

impl Solution {
    pub fn min_days(n: i32) -> i32 {
        let n = n as usize;

        let mut ans = usize::MAX;
        go(n, 0, &mut ans);
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
    #[case(10, 4)]
    #[case(6, 3)]
    #[case(100, 8)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::min_days(n);
        assert_eq!(actual, expected);
    }
}
