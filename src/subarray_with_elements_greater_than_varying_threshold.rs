//! Solution for https://leetcode.com/problems/subarray-with-elements-greater-than-varying-threshold
//! 2334. Subarray With Elements Greater Than Varying Threshold

fn calc_prev_bigger(a: &Vec<i32>) -> Vec<usize> {
    let n = a.len();
    let mut st = Vec::new();
    let mut len = vec![0; n];
    for i in 0..n {
        while !st.is_empty() && a[*st.last().unwrap()] >= a[i] {
            st.pop();
        }
        len[i] = if st.is_empty() {
            i
        } else {
            i - *st.last().unwrap() - 1
        };
        st.push(i);
    }
    len
}

impl Solution {
    pub fn valid_subarray_size(mut a: Vec<i32>, threshold: i32) -> i32 {
        let n = a.len();

        let prev_len = calc_prev_bigger(&a);

        a.reverse();
        let mut next_len = calc_prev_bigger(&a);
        next_len.reverse();
        a.reverse();

        for i in 0..n {
            let len = prev_len[i] + 1 + next_len[i];
            if a[i] > threshold / len as i32 {
                return len as i32;
            }
        }

        -1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,4,3,1], 6, 3)]
    #[case(vec![6,5,6,5,8], 7, 1)]
    fn case(#[case] nums: Vec<i32>, #[case] threshold: i32, #[case] expected: i32) {
        let actual = Solution::valid_subarray_size(nums, threshold);
        assert_eq!(actual, expected);
    }
}
