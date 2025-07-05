//! Solution for https://leetcode.com/problems/find-xor-sum-of-all-pairs-bitwise-and
//! 1835. Find XOR Sum of All Pairs Bitwise AND

const B: usize = 32;

impl Solution {
    pub fn get_xor_sum(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut ans = 0;
        for bit in 0..B {
            let mut cnt_a = 0;
            for &x in &a {
                if ((x >> bit) & 1) == 1 {
                    cnt_a += 1;
                }
            }
            let mut cnt_b = 0;
            for &x in &b {
                if ((x >> bit) & 1) == 1 {
                    cnt_b += 1;
                }
            }
            if cnt_a * cnt_b % 2 == 1 {
                ans ^= 1 << bit;
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
    #[case(vec![1,2,3], vec![6,5], 0)]
    #[case(vec![12], vec![4], 4)]
    fn case(#[case] arr1: Vec<i32>, #[case] arr2: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::get_xor_sum(arr1, arr2);
        assert_eq!(actual, expected);
    }
}
