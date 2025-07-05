//! Solution for https://leetcode.com/problems/count-subarrays-with-fixed-bounds
//! 2444. Count Subarrays With Fixed Bounds

fn solve_filtered(a: &Vec<i32>, min_k: i32, max_k: i32) -> u64 {
    let n = a.len();
    let mut next_max = vec![n; n + 1];
    for i in (0..n).rev() {
        if a[i] == max_k {
            next_max[i] = i;
        } else {
            next_max[i] = next_max[i + 1];
        }
    }

    let mut ans = 0;
    let mut prev_min = n;
    let mut prev_max = n;
    for i in 0..n {
        if a[i] == max_k {
            prev_max = i;
        }
        if a[i] == min_k {
            let L = if prev_min == n { 0 } else { prev_min + 1 };
            let R = n - 1;

            let mut good = (i - L + 1) as u64 * (R - i + 1) as u64;

            let l = if prev_max == n || prev_max < L {
                L
            } else {
                prev_max + 1
            };
            let mut r = next_max[i];
            if l < r {
                r -= 1;
                assert!(l <= i);
                assert!(i <= r);
                let bad = (i - l + 1) as u64 * (r - i + 1) as u64;
                good -= bad;
            }

            ans += good;

            prev_min = i;
        }
    }

    ans
}

impl Solution {
    pub fn count_subarrays(a: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let n = a.len();
        let mut ans = 0;
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && min_k <= a[j] && a[j] <= max_k {
                j += 1;
            }
            if i < j {
                let cur = solve_filtered(&a[i..j].to_vec(), min_k, max_k);
                ans += cur;
            }
            i = j + 1;
        }
        ans as i64
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,5,2,7,5], 1, 5, 2)]
    #[case(vec![1,1,1,1], 1, 1, 10)]
    fn case(#[case] nums: Vec<i32>, #[case] min_k: i32, #[case] max_k: i32, #[case] expected: i64) {
        let actual = Solution::count_subarrays(nums, min_k, max_k);
        assert_eq!(actual, expected);
    }
}
