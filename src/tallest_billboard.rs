//! Solution for https://leetcode.com/problems/tallest-billboard
//! 956. Tallest Billboard

impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let rods: Vec<_> = rods.iter().map(|&x| x as usize).collect();
        let n = rods.len();

        if n == 1 {
            return 0;
        }

        // also can be done like dp[diff] = biggest s0 with such diff from s1
        let M = rods.iter().sum::<usize>() / 2;
        let mut dp = vec![vec![false; M + 1]; M + 1];
        dp[0][0] = true;
        let mut pref = 0;
        for i in 0..n {
            let x = rods[i];
            for s0 in (0..=pref.min(M)).rev() {
                for s1 in (0..=(pref - s0).min(M)).rev() {
                    if dp[s0][s1] {
                        if s0 + x <= M {
                            dp[s0 + x][s1] = true;
                        }
                        if s1 + x <= M {
                            dp[s0][s1 + x] = true;
                        }
                    }
                }
            }
            pref += rods[i];
        }
        for ans in (0..=M).rev() {
            if dp[ans][ans] {
                return ans as i32;
            }
        }
        unreachable!()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,6], 6)]
    #[case(vec![1,2,3,4,5,6], 10)]
    #[case(vec![1,2], 0)]
    fn case(#[case] rods: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::tallest_billboard(rods);
        assert_eq!(actual, expected);
    }
}
