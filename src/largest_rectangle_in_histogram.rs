//! Solution for https://leetcode.com/problems/largest-rectangle-in-histogram
//! 84. Largest Rectangle in Histogram

fn calc_prev_less(h: &Vec<i32>) -> Vec<i32> {
    let mut st = Vec::<usize>::new();
    let mut prev_less = vec![-1; h.len()];
    for i in 0..h.len() {
        while let Some(&j) = st.last() {
            if h[j] >= h[i] {
                st.pop();
            } else {
                break;
            }
        }
        prev_less[i] = st.last().map(|&x| x as i32).unwrap_or(-1);
        st.push(i);
    }
    prev_less
}

impl Solution {
    pub fn largest_rectangle_area(mut h: Vec<i32>) -> i32 {
        let prev_less = calc_prev_less(&h);

        h.reverse();
        let next_less = calc_prev_less(&h);
        h.reverse();
        let next_less: Vec<_> = next_less
            .iter()
            .rev()
            .map(|idx| (h.len() as i32) - idx - 1)
            .collect();

        let mut ans = 0;
        for i in 0..h.len() {
            let l = prev_less[i];
            let r = next_less[i];
            assert!(l < (i as i32) && (i as i32) < r);
            let len = r - l - 1;
            ans = ans.max(len * h[i]);
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
    #[case(vec![2,1,5,6,2,3], 10)]
    #[case(vec![2,4], 4)]
    fn case(#[case] heights: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::largest_rectangle_area(heights);
        assert_eq!(actual, expected);
    }
}
