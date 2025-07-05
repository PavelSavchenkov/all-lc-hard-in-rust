//! Solution for https://leetcode.com/problems/split-array-largest-sum
//! 410. Split Array Largest Sum

fn can_split(a: &Vec<i32>, upper: i32, k: usize) -> bool {
    let mut segs = 1;
    let mut sum = 0;
    for i in 0..a.len() {
        if a[i] > upper {
            return false;
        }
        if sum + a[i] > upper {
            segs += 1;
            sum = 0;
        }
        sum += a[i];
    }
    assert!(sum <= upper);
    segs <= k
}

impl Solution {
    pub fn split_array(a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        let mut L = -1;
        let mut R = i32::pow(10, 9);
        while L + 1 != R {
            let M = (L + R) / 2;
            if can_split(&a, M, k) {
                R = M;
            } else {
                L = M;
            }
        }
        R
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![7,2,5,10,8], 2, 18)]
    #[case(vec![1,2,3,4,5], 2, 9)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::split_array(nums, k);
        assert_eq!(actual, expected);
    }
}
