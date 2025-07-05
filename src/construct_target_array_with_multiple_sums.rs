//! Solution for https://leetcode.com/problems/construct-target-array-with-multiple-sums
//! 1354. Construct Target Array With Multiple Sums

impl Solution {
    pub fn is_possible(a: Vec<i32>) -> bool {
        let n = a.len();
        if n == 1 {
            return a[0] == 1;
        }

        let mut a: Vec<i64> = a.iter().map(|&x| x as i64).collect();
        a.sort();
        a.reverse();

        let mut s = 0;
        let mut touched = Vec::new();
        for i in 0..n {
            if a[i] > 1 {
                touched.push(a[i]);
            }
            s += a[i];
        }
        if touched.len() > 40 {
            return false;
        }

        loop {
            let x = touched[0];
            if x == 1 {
                break;
            }
            let x1 = if touched.len() > 1 { touched[1] } else { 1 };
            assert!(x < s);
            assert!(x1 < s);
            let k = 2.max((s - x1) / (s - x));
            let xx = k * x - (k - 1) * s;
            if xx < 1 {
                return false;
            }
            s -= touched[0];
            touched[0] = xx;
            s += touched[0];

            touched.sort();
            touched.reverse();
        }
        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![9,3,5], true)]
    #[case(vec![1,1,1,2], false)]
    #[case(vec![8,5], true)]
    #[case(vec![1000000000,1], true)]
    fn case(#[case] target: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::is_possible(target);
        assert_eq!(actual, expected);
    }
}
