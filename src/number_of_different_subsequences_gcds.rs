//! Solution for https://leetcode.com/problems/number-of-different-subsequences-gcds
//! 1819. Number of Different Subsequences GCDs

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != 0 && b != 0 {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
    a + b
}

impl Solution {
    pub fn count_different_subsequence_gc_ds(a: Vec<i32>) -> i32 {
        let M = *a.iter().max().unwrap() as usize;
        let mut have = vec![false; M + 1];
        for &x in &a {
            have[x as usize] = true;
        }

        let mut ans = 0;
        for g in 1..=M {
            let mut actual = 0;
            for x in (g..=M).step_by(g) {
                if have[x] {
                    actual = gcd(actual, x);
                }
            }
            if actual == g {
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
    #[case(vec![6,10,3], 5)]
    #[case(vec![5,15,40,5,6], 7)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::count_different_subsequence_gc_ds(nums);
        assert_eq!(actual, expected);
    }
}
