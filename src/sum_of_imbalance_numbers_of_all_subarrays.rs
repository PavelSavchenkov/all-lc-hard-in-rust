//! Solution for https://leetcode.com/problems/sum-of-imbalance-numbers-of-all-subarrays
//! 2763. Sum of Imbalance Numbers of All Subarrays

impl Solution {
    pub fn sum_imbalance_numbers(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut ans = 0;
        for i in 0..n {
            let mut was = vec![false; n + 2];
            let mut cnt = 0; // cnt of x such that was[x] but !was[x + 1]
            for j in i..n {
                let x = a[j] as usize;
                if !was[x] {
                    if !was[x + 1] {
                        cnt += 1;
                    }
                    if x > 0 && was[x - 1] {
                        cnt -= 1;
                    }
                    was[x] = true;
                }
                ans += cnt - 1;
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
    #[case(vec![2,3,1,4], 3)]
    #[case(vec![1,3,3,3,5], 8)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::sum_imbalance_numbers(nums);
        assert_eq!(actual, expected);
    }
}
