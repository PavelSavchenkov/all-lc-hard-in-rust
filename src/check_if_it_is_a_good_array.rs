//! Solution for https://leetcode.com/problems/check-if-it-is-a-good-array
//! 1250. Check If It Is a Good Array

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 && b != 0 {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
    a + b
}

impl Solution {
    pub fn is_good_array(nums: Vec<i32>) -> bool {
        let g = nums.iter().fold(0, |acc, e| gcd(acc, *e));
        g == 1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![12,5,7,23], true)]
    #[case(vec![29,6,10], true)]
    #[case(vec![3,6], false)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::is_good_array(nums);
        assert_eq!(actual, expected);
    }
}
