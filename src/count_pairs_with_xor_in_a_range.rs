//! Solution for https://leetcode.com/problems/count-pairs-with-xor-in-a-range
//! 1803. Count Pairs With XOR in a Range

use std::collections::HashMap;

const B: usize = 32;

fn count(a: &Vec<i32>, bound: i32) -> i32 {
    let mut ans = 0;
    let mut cnt = vec![HashMap::new(); B + 1];
    for &x in a {
        for i in 0..B {
            if ((bound >> i) & 1) == 1 {
                let need_xor = (bound >> i) ^ 1;
                let need_other = (x >> i) ^ need_xor;
                ans += cnt[B - i].get(&need_other).unwrap_or(&0);
            }
        }

        for len in 1..=B {
            let xx = x >> (B - len);
            *cnt[len].entry(xx).or_insert(0) += 1;
        }
    }
    ans
}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        count(&nums, high + 1) - count(&nums, low)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,4,2,7], 2, 6, 6)]
    #[case(vec![9,8,4,2,1], 5, 14, 8)]
    fn case(#[case] nums: Vec<i32>, #[case] low: i32, #[case] high: i32, #[case] expected: i32) {
        let actual = Solution::count_pairs(nums, low, high);
        assert_eq!(actual, expected);
    }
}
