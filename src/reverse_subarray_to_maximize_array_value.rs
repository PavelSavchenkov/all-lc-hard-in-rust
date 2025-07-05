//! Solution for https://leetcode.com/problems/reverse-subarray-to-maximize-array-value
//! 1330. Reverse Subarray To Maximize Array Value

fn coef(sign: usize) -> i32 {
    match sign {
        0 => -1,
        1 => 1,
        _ => panic!(),
    }
}

impl Solution {
    pub fn max_value_after_reverse(a: Vec<i32>) -> i32 {
        // |x - y| = max(x - y, y - x)
        let n = a.len();

        let mut ans = 0;
        let mut max_func = vec![vec![i32::MIN; 2]; 2];
        for i in 1..n - 1 {
            let r = i;
            for sign1 in 0..2 {
                for sign2 in 0..2 {
                    let cur =
                        -(a[r] - a[r + 1]).abs() + coef(sign1) * a[r] + coef(sign2) * a[r + 1];
                    let prev = max_func[sign1 ^ 1][sign2 ^ 1];
                    if prev > i32::MIN {
                        let cur_ans = cur + prev;
                        ans = ans.max(cur_ans);
                    }
                }
            }

            let l = i;
            for sign1 in 0..2 {
                for sign2 in 0..2 {
                    let cur =
                        -(a[l - 1] - a[l]).abs() + coef(sign1) * a[l - 1] + coef(sign2) * a[l];
                    max_func[sign1][sign2] = max_func[sign1][sign2].max(cur);
                }
            }
        }

        // pref
        for i in 1..n - 1 {
            let cur = -(a[i] - a[i + 1]).abs() + (a[0] - a[i + 1]).abs();
            ans = ans.max(cur);
        }

        // suff
        for i in 1..n - 1 {
            let cur = -(a[i] - a[i - 1]).abs() + (a[n - 1] - a[i - 1]).abs();
            ans = ans.max(cur);
        }

        let mut orig = 0;
        for i in 0..n - 1 {
            orig += (a[i] - a[i + 1]).abs();
        }
        ans += orig;
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
    #[case(vec![2,3,1,5,4], 10)]
    #[case(vec![2,4,9,24,2,1,10], 68)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_value_after_reverse(nums);
        assert_eq!(actual, expected);
    }
}
