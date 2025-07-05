//! Solution for https://leetcode.com/problems/earliest-second-to-mark-indices-ii
//! 3049. Earliest Second to Mark Indices II

impl Solution {
    pub fn earliest_second_to_mark_indices(a: Vec<i32>, mut change_indices: Vec<i32>) -> i32 {
        let n = a.len();
        let m = change_indices.len();

        {
            let mut was = vec![false; n];
            for i in &mut change_indices {
                *i -= 1;
                if was[*i as usize] {
                    *i = -1;
                } else {
                    was[*i as usize] = true;
                }
            }
        }

        let S = a.iter().fold(0, |acc, e| acc + *e as i64);
        // dp[i][cnt] -- we passed "i" items in change_indices and
        // there are "cnt" indices which are not marked and which were made zero by operation 2
        let mut dp = vec![i64::MAX; n + 1];
        dp[0] = 0;
        for i in 0..=m {
            if i as i64 - S - n as i64 >= dp[0] {
                return i as i32;
            }
            if i == m {
                break;
            }
            let mut ndp = vec![i64::MAX; n + 1];
            for cnt in 0..=i.min(n) {
                let cur_dp = dp[cnt];
                if cur_dp == i64::MAX {
                    continue;
                }
                // close one of unclosed
                if cnt > 0 {
                    ndp[cnt - 1] = ndp[cnt - 1].min(cur_dp);
                }
                // turn change_indices[i] into zero
                if change_indices[i] != -1 {
                    ndp[cnt + 1] =
                        ndp[cnt + 1].min(cur_dp + 1 - a[change_indices[i] as usize] as i64);
                }
                // do "nothing"
                ndp[cnt] = ndp[cnt].min(cur_dp);
            }
            dp = ndp;
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
    #[case(vec![3,2,3], vec![1,3,2,2,2,2,3], 6)]
    #[case(vec![0,0,1,2], vec![1,2,1,2,1,2,1,2], 7)]
    #[case(vec![1,2,3], vec![1,2,3], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] change_indices: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::earliest_second_to_mark_indices(nums, change_indices);
        assert_eq!(actual, expected);
    }
}
