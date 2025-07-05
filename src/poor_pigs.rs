//! Solution for https://leetcode.com/problems/poor-pigs
//! 458. Poor Pigs

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let max_rounds = minutes_to_test as usize / minutes_to_die as usize;
        let max_buckets = buckets as usize;
        let max_pigs = 10;

        let mut binom = vec![vec![0; max_pigs + 1]; max_pigs + 1];
        for n in 0..binom.len() {
            binom[n][0] = 1;
            for k in 1..=n {
                binom[n][k] = binom[n - 1][k] + binom[n - 1][k - 1];
            }
        }

        let mut dp = vec![vec![usize::MAX; max_rounds + 1]; max_buckets + 1];
        for rounds in 0..=max_rounds {
            for buckets in 0..=max_buckets {
                if buckets <= 1 {
                    dp[buckets][rounds] = 0;
                    continue
                }
                if rounds == 0 {
                    continue
                }
                let mut cur_max_pigs = 1;
                while (1 << cur_max_pigs) < buckets {
                    cur_max_pigs += 1;
                }
                for pigs in 1..=cur_max_pigs {
                    let mut covered = 0;
                    let mut b = buckets;
                    for cnt in 0..=pigs {
                        let coef = binom[pigs][cnt];
                        // max b | dp[b][rounds - 1] <= pigs - cnt
                        while b > 0 && dp[b - 1][rounds - 1] > pigs - cnt {
                            b -= 1;
                        }
                        let cur_covered = if dp[b][rounds - 1] <= pigs - cnt { b } else { b - 1 };
                        covered += cur_covered * coef;
                    } 
                    if covered >= buckets {
                        dp[buckets][rounds] = pigs;
                        break
                    }
                }
            }
        }

        dp[max_buckets][max_rounds] as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, 15, 15, 2)]
    #[case(4, 15, 30, 2)]
    fn case(
        #[case] buckets: i32,
        #[case] minutes_to_die: i32,
        #[case] minutes_to_test: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test);
        assert_eq!(actual, expected);
    }
}
