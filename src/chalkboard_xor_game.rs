//! Solution for https://leetcode.com/problems/chalkboard-xor-game
//! 810. Chalkboard XOR Game

impl Solution {
    pub fn xor_game(a: Vec<i32>) -> bool {
        // invariant -- the number of numbers which occured odd number of times is odd
        // base losing position -- odd number of equal numbers
        let n = a.len();
        let xor = a.iter().fold(0, |acc, e| acc ^ e);
        if xor == 0 {
            return true;
        }
        n % 2 == 0
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,1,2], false)]
    #[case(vec![0,1], true)]
    #[case(vec![1,2,3], true)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::xor_game(nums);
        assert_eq!(actual, expected);
    }
}
