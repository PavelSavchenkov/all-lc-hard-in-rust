//! Solution for https://leetcode.com/problems/self-crossing
//! 335. Self Crossing

fn check4(d: &[i32]) -> bool {
    d[3] >= d[1] && d[2] <= d[0]
}

fn check6(d: &[i32]) -> bool {
    d[2] > d[0] && d[3] >= d[1] && d[4] >= d[2] - d[0] && d[4] <= d[2] && d[5] >= d[3] - d[1]
}

impl Solution {
    pub fn is_self_crossing(mut d: Vec<i32>) -> bool {
        d.push(0);
        let n = d.len();
        for i in 0..n {
            if i + 4 <= n && check4(&d[i..i + 4]) {
                return true;
            }
            if i + 6 <= n && check6(&d[i..i + 6]) {
                return true;
            }
        }
        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,1,1,2], true)]
    #[case(vec![1,2,3,4], false)]
    #[case(vec![1,1,1,2,1], true)]
    fn case(#[case] distance: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::is_self_crossing(distance);
        assert_eq!(actual, expected);
    }
}
