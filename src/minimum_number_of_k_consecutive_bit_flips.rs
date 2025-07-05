//! Solution for https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips
//! 995. Minimum Number of K Consecutive Bit Flips

impl Solution {
    pub fn min_k_bit_flips(mut a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        a.insert(0, 1);
        a.push(1);
        let k = k as usize;

        let mut b = vec![0; n + 1];
        for i in 0..=n {
            if a[i] != a[i + 1] {
                b[i] = 1;
            }
        }

        let mut ans = 0;
        for rem in 0..k {
            let mut flat = Vec::new();
            for l in (rem..=n).step_by(k) {
                flat.push(b[l]);
            }
            assert!(!flat.is_empty());
            for i in 0..flat.len() - 1 {
                if flat[i] == 1 {
                    ans += 1;
                    flat[i] ^= 1;
                    flat[i + 1] ^= 1;
                }
            }
            if *flat.last().unwrap() == 1 {
                return -1;
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
    #[case(vec![0,1,0], 1, 2)]
    #[case(vec![1,1,0], 2, -1)]
    #[case(vec![0,0,0,1,0,1,1,0], 3, 3)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::min_k_bit_flips(nums, k);
        assert_eq!(actual, expected);
    }
}
