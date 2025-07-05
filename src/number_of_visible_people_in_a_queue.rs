//! Solution for https://leetcode.com/problems/number-of-visible-people-in-a-queue
//! 1944. Number of Visible People in a Queue

impl Solution {
    pub fn can_see_persons_count(mut a: Vec<i32>) -> Vec<i32> {
        a.reverse();
        let n = a.len();
        let mut ans = vec![0; n];
        let mut st = Vec::new();
        for i in 0..n {
            let mut cur = 0;
            while !st.is_empty() && a[i] > a[*st.last().unwrap()] {
                cur += 1;
                st.pop();
            }
            if !st.is_empty() {
                cur += 1;
            }
            st.push(i);
            ans[i] = cur;
        }
        ans.reverse();
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
    #[case(vec![10,6,8,5,11,9], vec![3,1,2,1,1,0])]
    #[case(vec![5,1,2,3,10], vec![4,1,1,1,0])]
    fn case(#[case] heights: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::can_see_persons_count(heights);
        assert_eq!(actual, expected);
    }
}
