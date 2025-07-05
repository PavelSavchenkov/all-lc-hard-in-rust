//! Solution for https://leetcode.com/problems/couples-holding-hands
//! 765. Couples Holding Hands

impl Solution {
    pub fn min_swaps_couples(mut a: Vec<i32>) -> i32 {
        for x in &mut a {
            *x /= 2;
        }
        let n = a.len();

        let mut ans = 0;
        loop {
            let mut found = false;
            for i in (0..n).step_by(2) {
                if a[i] != a[i + 1] {
                    for j in 0..n {
                        if j != i && a[j] == a[i] {
                            ans += 1;
                            a.swap(i ^ 1, j);
                            break;
                        }
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                break;
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
    #[case(vec![0,2,1,3], 1)]
    #[case(vec![3,2,0,1], 0)]
    fn case(#[case] row: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_swaps_couples(row);
        assert_eq!(actual, expected);
    }
}
