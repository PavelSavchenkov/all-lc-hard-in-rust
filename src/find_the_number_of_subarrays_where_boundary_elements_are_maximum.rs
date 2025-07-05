//! Solution for https://leetcode.com/problems/find-the-number-of-subarrays-where-boundary-elements-are-maximum
//! 3113. Find the Number of Subarrays Where Boundary Elements Are Maximum

impl Solution {
    pub fn number_of_subarrays(a: Vec<i32>) -> i64 {
        let n = a.len();

        let mut ans = 0;
        let mut st = Vec::<(i32, usize)>::new();
        for &x in &a {
            while !st.is_empty() && x > st.last().unwrap().0 {
                st.pop();
            }
            if !st.is_empty() && st.last().unwrap().0 == x {
                ans += st.last().unwrap().1 as i64;
                st.last_mut().unwrap().1 += 1;
            } else {
                st.push((x, 1));
            }
            ans += 1;
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
    #[case(vec![1,4,3,3,2], 6)]
    #[case(vec![3,3,3], 6)]
    #[case(vec![1], 1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::number_of_subarrays(nums);
        assert_eq!(actual, expected);
    }
}
