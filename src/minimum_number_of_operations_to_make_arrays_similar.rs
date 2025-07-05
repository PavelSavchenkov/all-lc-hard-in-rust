//! Solution for https://leetcode.com/problems/minimum-number-of-operations-to-make-arrays-similar
//! 2449. Minimum Number of Operations to Make Arrays Similar

fn split(a: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut even = Vec::new();
    let mut odd = Vec::new();
    for &x in a {
        if x % 2 == 0 {
            even.push(x);
        } else {
            odd.push(x);
        }
    }
    even.sort();
    odd.sort();
    (odd, even)
}

impl Solution {
    pub fn make_similar(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let (nums1, nums2) = split(&nums);
        let (target1, target2) = split(&target);

        let mut ans: i64 = 0;
        for i in 0..nums1.len() {
            ans += (nums1[i] - target1[i]).abs() as i64;
        }
        for i in 0..nums2.len() {
            ans += (nums2[i] - target2[i]).abs() as i64;
        }
        ans / 4
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![8,12,6], vec![2,14,10], 2)]
    #[case(vec![1,2,5], vec![4,1,3], 1)]
    #[case(vec![1,1,1,1,1], vec![1,1,1,1,1], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] target: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::make_similar(nums, target);
        assert_eq!(actual, expected);
    }
}
