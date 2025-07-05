//! Solution for https://leetcode.com/problems/kth-smallest-instructions
//! 1643. Kth Smallest Instructions

impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let mut v = destination[0] as usize;
        let mut h = destination[1] as usize;
        let mut k = k as u64;

        let N = h + v;
        let mut binom = vec![vec![0 as u64; N]; N];
        for n in 0..N {
            binom[n][0] = 1;
            for k in 1..=n {
                binom[n][k] = binom[n - 1][k - 1] + binom[n - 1][k];
            }
        }

        let mut ans = String::new();
        while v > 0 || h > 0 {
            let if_h = binom[v + h - 1][v];
            if k <= if_h {
                ans.push('H');
                h -= 1;
            } else {
                k -= if_h;
                v -= 1;
                ans.push('V');
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
    #[case(vec![2,3], 1, "HHHVV")]
    #[case(vec![2,3], 2, "HHVHV")]
    #[case(vec![2,3], 3, "HHVVH")]
    fn case(#[case] destination: Vec<i32>, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::kth_smallest_path(destination, k);
        assert_eq!(actual, expected);
    }
}
