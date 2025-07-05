//! Solution for https://leetcode.com/problems/find-a-value-of-a-mysterious-function-closest-to-target
//! 1521. Find a Value of a Mysterious Function Closest to Target

impl Solution {
    pub fn closest_to_target(a: Vec<i32>, target: i32) -> i32 {
        let n = a.len();

        let mut ans = i32::MAX;
        let mut next = vec![n; 32];
        for i in (0..n).rev() {
            for b in 0..32 {
                if ((a[i] >> b) & 1) == 0 {
                    next[b] = i;
                }
            }

            let mut ord: Vec<_> = (0..32).collect();
            ord.sort_by_key(|&j| next[j]);

            let mut cur = i32::MAX;
            for b in ord {
                let j = next[b];
                if j == n {
                    break;
                }
                cur &= a[j];
                ans = ans.min((target - cur).abs());
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
    #[case(vec![9,12,3,7,15], 5, 2)]
    #[case(vec![1000000,1000000,1000000], 1, 999999)]
    #[case(vec![1,2,4,8,16], 0, 0)]
    fn case(#[case] arr: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::closest_to_target(arr, target);
        assert_eq!(actual, expected);
    }
}
