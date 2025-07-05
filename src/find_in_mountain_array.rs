//! Solution for https://leetcode.com/problems/find-in-mountain-array
//! 1095. Find in Mountain Array

/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct MountainArray;
 *  impl MountainArray {
 *     fn get(index:i32)->i32;
 *     fn length()->i32;
 * };
 */

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
        let n = mountainArr.length();

        // find the peak
        let mut L = 0;
        let mut R = n - 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if mountainArr.get(M) < mountainArr.get(M + 1) {
                L = M;
            } else {
                R = M;
            }
        }
        let mid = R;

        if mountainArr.get(mid) == target {
            return mid;
        }

        let mut L = 0;
        let mut R = mid;
        while L + 1 != R {
            let M = (L + R) / 2;
            if mountainArr.get(M) <= target {
                L = M;
            } else {
                R = M;
            }
        }
        if mountainArr.get(L) == target {
            return L;
        }

        let mut L = mid;
        let mut R = n;
        while L + 1 != R {
            let M = (L + R) / 2;
            if mountainArr.get(M) >= target {
                L = M;
            } else {
                R = M;
            }
        }
        if mountainArr.get(L) == target {
            return L;
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
    #[case(todo!("[1,2,3,4,5,3,1]"), todo!("3"), 2)]
    #[case(todo!("[0,1,2,4,2,1]"), todo!("3"), -1)]
    fn case(#[case] target: i32, #[case] mountainArr: &MountainArray, #[case] expected: i32) {
        let actual = Solution::find_in_mountain_array(target, mountainArr);
        assert_eq!(actual, expected);
    }
}
