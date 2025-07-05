//! Solution for https://leetcode.com/problems/distribute-repeating-integers
//! 1655. Distribute Repeating Integers

impl Solution {
    pub fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
        let max = *nums.iter().max().unwrap() as usize;
        let mut cnt = vec![0; max + 1];
        for &x in &nums {
            cnt[x as usize] += 1;
        }
        cnt.sort();
        cnt.reverse();

        let m = quantity.len();
        cnt.truncate(m);

        let mut dp = vec![false; 1 << m];
        dp[0] = true;
        for &c in &cnt {
            for mask in (0..1 << m).rev() {
                if !dp[mask] {
                    continue;
                }
                let not_mask = !mask & ((1 << m) - 1);
                let mut sub = not_mask;
                while sub != 0 {
                    let mut sum = 0;
                    for i in 0..m {
                        if ((sub >> i) & 1) == 1 {
                            sum += quantity[i];
                        }
                    }
                    if sum <= c {
                        dp[mask | sub] = true;
                    }
                    sub = (sub - 1) & not_mask;
                }
            }
        }
        dp[(1 << m) - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4], vec![2], false)]
    #[case(vec![1,2,3,3], vec![2], true)]
    #[case(vec![1,1,2,2], vec![2,2], true)]
    fn case(#[case] nums: Vec<i32>, #[case] quantity: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::can_distribute(nums, quantity);
        assert_eq!(actual, expected);
    }
}
