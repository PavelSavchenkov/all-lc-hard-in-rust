//! Solution for https://leetcode.com/problems/sum-of-floored-pairs
//! 1862. Sum of Floored Pairs

impl Solution {
    pub fn sum_of_floored_pairs(a: Vec<i32>) -> i32 {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();

        let M = *a.iter().max().unwrap();
        let mut cnt = vec![0; M + 1];
        for &x in &a {
            cnt[x] += 1;
        }
        let mut suff = cnt.clone();
        for x in (0..M).rev() {
            suff[x] += suff[x + 1];
        }

        let mut ans: u64 = 0;
        for x in 1..=M {
            // number of pairs with floor at least i
            for i in 1..=M {
                let up = x * i;
                if up > M {
                    break;
                }
                ans += suff[up] as u64 * cnt[x] as u64;
            }
        }
        (ans % 1_000_000_007) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,5,9], 10)]
    #[case(vec![7,7,7,7,7,7,7], 49)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::sum_of_floored_pairs(nums);
        assert_eq!(actual, expected);
    }
}
