//! Solution for https://leetcode.com/problems/find-k-th-smallest-pair-distance
//! 719. Find K-th Smallest Pair Distance

fn cnt_less_or_eq(a: &Vec<i32>, bound: i32) -> u32 {
    if bound < 0 {
        return 0;
    }
    let mut j = 0;
    let mut ans = 0;
    for i in 0..a.len() {
        while a[i] - a[j] > bound {
            j += 1;
        }
        assert!(j <= i);
        ans += i - j;
    }
    ans as u32
}

impl Solution {
    pub fn smallest_distance_pair(mut a: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        let n = a.len();
        a.sort();

        let mut L = -1;
        let mut R = a[n - 1] - a[0] + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if cnt_less_or_eq(&a, M) >= k {
                R = M
            } else {
                L = M
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
    #[case(vec![1,3,1], 1, 0)]
    #[case(vec![1,1,1], 2, 0)]
    #[case(vec![1,6,1], 3, 5)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::smallest_distance_pair(nums, k);
        assert_eq!(actual, expected);
    }
}
