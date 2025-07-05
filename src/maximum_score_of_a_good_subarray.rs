//! Solution for https://leetcode.com/problems/maximum-score-of-a-good-subarray
//! 1793. Maximum Score of a Good Subarray

fn calc_left(a: &Vec<i32>) -> Vec<usize> {
    let n = a.len();
    let mut ans = vec![0; n];
    let mut st = Vec::new();
    for i in 0..n {
        while !st.is_empty() && a[i] <= a[*st.last().unwrap()] {
            st.pop();
        }
        if st.is_empty() {
            ans[i] = i;
        } else {
            ans[i] = i - *st.last().unwrap() - 1;
        }
        st.push(i);
    }
    ans
}

impl Solution {
    pub fn maximum_score(mut a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let k = k as usize;

        let left = calc_left(&a);

        a.reverse();
        let mut right = calc_left(&a);
        right.reverse();
        a.reverse();

        let mut ans = 0;
        for i in 0..n {
            let l = i - left[i];
            let r = i + right[i];
            assert!(r < n);
            if l <= k && k <= r {
                ans = ans.max(a[i] * (r - l + 1) as i32);
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
    #[case(vec![1,4,3,7,4,5], 3, 15)]
    #[case(vec![5,5,4,5,4,1,1,1], 0, 20)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::maximum_score(nums, k);
        assert_eq!(actual, expected);
    }
}
