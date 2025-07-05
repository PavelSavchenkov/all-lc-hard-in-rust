//! Solution for https://leetcode.com/problems/patching-array
//! 330. Patching Array

/*
proof:
consider any sorted array which covers [0; n]
i claim that a[0] ... a[i] cover [0; a[0] + ... + a[i]] for every i
*/

impl Solution {
    pub fn min_patches(a: Vec<i32>, n: i32) -> i32 {
        let a: Vec<_> = a.iter().map(|&x| x as u64).collect();
        let n = n as u64;

        let mut sum: u64 = 0;
        let mut ans = 0;
        let mut i = 0;
        while sum < n && i < a.len() {
            if a[i] <= sum + 1 {
                sum += a[i];
                i += 1;
            } else {
                ans += 1;
                sum += sum + 1;
            }
        }
        while sum < n {
            sum += sum + 1;
            ans += 1;
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
    #[case(vec![1,3], 6, 1)]
    #[case(vec![1,5,10], 20, 2)]
    #[case(vec![1,2,2], 5, 0)]
    fn case(#[case] nums: Vec<i32>, #[case] n: i32, #[case] expected: i32) {
        let actual = Solution::min_patches(nums, n);
        assert_eq!(actual, expected);
    }
}
