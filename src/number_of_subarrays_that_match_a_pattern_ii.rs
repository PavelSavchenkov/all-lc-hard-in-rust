//! Solution for https://leetcode.com/problems/number-of-subarrays-that-match-a-pattern-ii
//! 3036. Number of Subarrays That Match a Pattern II

fn calc_z(s: &Vec<i32>) -> Vec<usize> {
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
    pub fn count_matching_subarrays(mut a: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let n = a.len();

        let mut na = Vec::with_capacity(n - 1);
        for i in 0..n - 1 {
            na.push((a[i + 1] - a[i]).signum());
        }
        a = na;

        let mut s = pattern.clone();
        s.extend(a.iter());

        let z = calc_z(&s);

        let mut ans = 0;
        for i in pattern.len()..s.len() {
            if z[i] >= pattern.len() {
                ans += 1;
            }
        }
        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4,5,6], vec![1,1], 4)]
    #[case(vec![1,4,4,1,3,5,5,3], vec![1,0,-1], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] pattern: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::count_matching_subarrays(nums, pattern);
        assert_eq!(actual, expected);
    }
}
