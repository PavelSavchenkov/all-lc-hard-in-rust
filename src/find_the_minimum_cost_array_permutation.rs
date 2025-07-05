//! Solution for https://leetcode.com/problems/find-the-minimum-cost-array-permutation
//! 3149. Find the Minimum Cost Array Permutation

fn bit(mask: usize, b: usize) -> bool {
    ((mask >> b) & 1) == 1
}

impl Solution {
    pub fn find_permutation(a: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();

        let mut masks: Vec<_> = (1..((1 << n) as usize) - 1).collect();
        masks.sort_by_key(|&m| m.count_ones());

        let mut best_perm: Vec<_> = (0..n).collect();
        let mut best_sum = usize::MAX;
        for p0 in 0..n {
            let mut dp = vec![vec![usize::MAX; n]; 1 << n];
            let mut prev = vec![vec![n; n]; 1 << n];
            dp[1 << p0][p0] = 0;

            for &used in &masks {
                let i = n - used.count_ones() as usize;
                for last in 0..n {
                    if !bit(used, last) {
                        continue;
                    }
                    let cur_dp = dp[used][last];
                    if cur_dp == usize::MAX {
                        continue;
                    }
                    for p in 0..n {
                        if bit(used, p) {
                            continue;
                        }
                        let mut ndp = cur_dp;
                        ndp += (p as i32 - a[last] as i32).abs() as usize;
                        if i == 1 {
                            ndp += (p0 as i32 - a[p] as i32).abs() as usize;
                        }
                        let nused = used ^ (1 << p);
                        if ndp < dp[nused][p] || (ndp == dp[nused][p] && last < prev[nused][p]) {
                            dp[nused][p] = ndp;
                            prev[nused][p] = last;
                        }
                    }
                }
            }

            let mut best_p = n;
            let mut min_sum = usize::MAX;
            let full = (1 << n) - 1;
            for p in 0..n {
                let cur_dp = dp[full][p];
                if cur_dp < min_sum {
                    min_sum = cur_dp;
                    best_p = p;
                }
            }

            assert!(best_p < n);
            let mut p = best_p;
            let mut mask = full;
            let mut perm = Vec::new();
            perm.push(p0);
            for i in 1..n {
                perm.push(p);
                let np = prev[mask][p];
                mask ^= 1 << p;
                p = np;
            }
            assert!(p == p0);

            // eprintln!(
            //     "p0 = {}, min_sum = {}, best_perm = {:#?}",
            //     p0, min_sum, best_perm
            // );

            if min_sum < best_sum || (min_sum == best_sum && perm < best_perm) {
                best_sum = min_sum;
                best_perm = perm;
            }
        }

        let ans: Vec<_> = best_perm.iter().map(|&x| x as i32).collect();
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
    #[case(vec![1,0,2], vec![0,1,2])]
    #[case(vec![0,2,1], vec![0,2,1])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::find_permutation(nums);
        assert_eq!(actual, expected);
    }
}
