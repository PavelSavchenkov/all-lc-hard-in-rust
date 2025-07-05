//! Solution for https://leetcode.com/problems/trapping-rain-water
//! 42. Trapping Rain Water

impl Solution {
    pub fn trap(h: Vec<i32>) -> i32 {
        let n = h.len();
        let mut pref_max = vec![0 as i32; n];
        let mut suff_max = vec![0 as i32; n];

        let mut mx = h[0];
        for i in 0..n {
            mx = mx.max(h[i]);
            pref_max[i] = mx;
        }

        mx = h[n - 1];
        for i in (0..n).rev() {
            mx = mx.max(h[i]);
            suff_max[i] = mx;
        }

        let mut ans = 0;
        for i in 0..n {
            let up_to = pref_max[i].min(suff_max[i]);
            ans += 0.max(up_to - h[i]);
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
    #[case(vec![0,1,0,2,1,0,1,3,2,1,2,1], 6)]
    #[case(vec![4,2,0,3,2,5], 9)]
    fn case(#[case] height: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::trap(height);
        assert_eq!(actual, expected);
    }
}
