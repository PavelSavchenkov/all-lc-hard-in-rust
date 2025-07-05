//! Solution for https://leetcode.com/problems/create-maximum-number
//! 321. Create Maximum Number

impl Solution {
    pub fn max_number(mut a: Vec<i32>, mut b: Vec<i32>, k: i32) -> Vec<i32> {
        let n = a.len();
        let m = b.len();
        let k = k as usize;
        a.push(-1);
        b.push(-1);

        let mut ans = Vec::new();
        let mut dp = vec![vec![true; m + 1]; n + 1];
        let mut ndp = vec![vec![false; m + 1]; n + 1];
        let mut positions = Vec::with_capacity((n + 1) * (m + 1));
        for len in 0..k {
            let mut best_dig = -1;
            for i in 0..=n {
                for j in 0..=m {
                    if !dp[i][j] {
                        continue;
                    }
                    if len + (n - i) + (m - j) < k {
                        break;
                    }
                    for ((ni, nj), d) in [((i + 1, j), a[i]), ((i, j + 1), b[j])] {
                        if d < best_dig {
                            continue;
                        }
                        if d > best_dig {
                            positions.clear();
                            best_dig = d;
                        }
                        positions.push((ni, nj));
                    }
                }
            }

            assert!(best_dig >= 0);
            ans.push(best_dig);

            for (i, j) in &positions {
                ndp[*i][*j] = true;
            }
            for i in 0..=n {
                for j in 0..=m {
                    if !ndp[i][j] {
                        continue;
                    }
                    if i + 1 <= n {
                        ndp[i + 1][j] = true;
                    }
                    if j + 1 <= m {
                        ndp[i][j + 1] = true;
                    }
                }
                dp[i].fill(false);
            }
            std::mem::swap(&mut dp, &mut ndp);
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
    #[case(vec![3,4,6,5], vec![9,1,2,5,8,3], 5, vec![9,8,6,5,3])]
    #[case(vec![6,7], vec![6,0,4], 5, vec![6,7,6,0,4])]
    #[case(vec![3,9], vec![8,9], 3, vec![9,8,9])]
    fn case(
        #[case] nums1: Vec<i32>,
        #[case] nums2: Vec<i32>,
        #[case] k: i32,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::max_number(nums1, nums2, k);
        assert_eq!(actual, expected);
    }
}
