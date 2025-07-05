//! Solution for https://leetcode.com/problems/smallest-rotation-with-highest-score
//! 798. Smallest Rotation with Highest Score

impl Solution {
    pub fn best_rotation(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut pref = vec![0; n + 1];
        let mut add_seg = |l: usize, r: usize| {
            pref[l] += 1;
            pref[r] -= 1;
        };
        for i in 0..n {
            let x = a[i] as usize;
            if x <= i {
                let max_k = i - x;
                add_seg(0, max_k + 1);
            }
            if x <= n - 1 {
                let max_k = (i + 1) + (n - 1) - x;
                let min_k = i + 1;
                add_seg(min_k, max_k.min(n - 1) + 1);
            }
        }
        for i in 1..=n {
            pref[i] += pref[i - 1]
        }
        let mx = pref.iter().max().unwrap();
        let pos = pref.iter().position(|sum| sum == mx).unwrap();
        pos as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,1,4,0], 3)]
    #[case(vec![1,3,0,2,4], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::best_rotation(nums);
        assert_eq!(actual, expected);
    }
}
