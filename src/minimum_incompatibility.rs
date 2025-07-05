//! Solution for https://leetcode.com/problems/minimum-incompatibility
//! 1681. Minimum Incompatibility

fn remin(a: &mut usize, b: usize) {
    if *a > b {
        *a = b;
    }
}

impl Solution {
    pub fn minimum_incompatibility(a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let max_a = *a.iter().max().unwrap();

        let mut incompatibility = vec![usize::MAX; 1 << n];
        for mask in 0..((1 << n) as usize) {
            if mask.count_ones() as usize != n / k {
                continue;
            }
            let mut min = usize::MAX;
            let mut max = usize::MIN; // 0
            let mut was = vec![false; max_a + 1];
            let mut valid = true;
            for i in 0..n {
                if ((mask >> i) & 1) == 1 {
                    if was[a[i]] {
                        valid = false;
                        break;
                    }
                    was[a[i]] = true;
                    min = min.min(a[i]);
                    max = max.max(a[i]);
                }
            }
            if valid {
                incompatibility[mask] = max - min;
            }
        }

        let mut dp = vec![usize::MAX; 1 << n];
        dp[0] = 0;
        for mask in 0..((1 << n) as usize) {
            let cur_dp = dp[mask];
            if cur_dp == usize::MAX {
                continue;
            }
            assert!(mask.count_ones() as usize % (n / k) == 0);
            let not_mask = !mask & ((1 << n) - 1);
            let mut sub = not_mask;
            while sub != 0 {
                if sub.count_ones() as usize == n / k {
                    let add = incompatibility[sub];
                    if add < usize::MAX {
                        assert!((sub & mask) == 0);
                        remin(&mut dp[mask | sub], cur_dp + add);
                    }
                }
                sub = (sub - 1) & not_mask;
            }
        }
        dp[(1 << n) - 1] as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,1,4], 2, 4)]
    #[case(vec![6,3,8,1,3,1,2,2], 4, 6)]
    #[case(vec![5,3,3,6,3,3], 3, -1)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::minimum_incompatibility(nums, k);
        assert_eq!(actual, expected);
    }
}
