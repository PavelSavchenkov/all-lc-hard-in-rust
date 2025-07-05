//! Solution for https://leetcode.com/problems/triples-with-bitwise-and-equal-to-zero
//! 982. Triples with Bitwise AND Equal To Zero

impl Solution {
    pub fn count_triplets(a: Vec<i32>) -> i32 {
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let n = a.len();
        let M = a.iter().fold(0, |acc, e| acc | e);

        let mut dp1 = vec![0; M + 1];
        let mut dp2 = vec![0; M + 1];
        let mut ans = 0;
        for &x in &a {
            for mask in 0..=M {
                if (mask & x) == 0 {
                    ans += dp2[mask] * 6;
                }
            }
            for mask in 0..=M {
                dp2[mask & x] += dp1[mask];
            }
            dp1[x] += 1;
        }
        for i in 0..n {
            for j in 0..n {
                if i != j && (a[i] & a[j]) == 0 {
                    ans += 3;
                }
            }
            if a[i] == 0 {
                ans += 1;
            }
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
    #[case(vec![2,1,3], 12)]
    #[case(vec![0,0,0], 27)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::count_triplets(nums);
        assert_eq!(actual, expected);
    }
}
