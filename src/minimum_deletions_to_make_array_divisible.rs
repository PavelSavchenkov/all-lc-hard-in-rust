//! Solution for https://leetcode.com/problems/minimum-deletions-to-make-array-divisible
//! 2344. Minimum Deletions to Make Array Divisible

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let mut g = 0;
        for x in nums_divide {
            g = gcd(x as u32, g);
        }

        nums.sort();
        for i in 0..nums.len() {
            if g % nums[i] as u32 == 0 {
                return i as i32;
            }
        }

        -1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,2,4,3], vec![9,6,9,3,15], 2)]
    #[case(vec![4,3,6], vec![8,2,6,10], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] nums_divide: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_operations(nums, nums_divide);
        assert_eq!(actual, expected);
    }
}
