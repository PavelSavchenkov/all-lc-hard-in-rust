//! Solution for https://leetcode.com/problems/find-the-k-sum-of-an-array
//! 2386. Find the K-Sum of an Array

fn rec(i: usize, a: &Vec<i64>, rem: &mut usize, cur_sum: i64, upper: i64) {
    if i == a.len() || *rem == 0 || cur_sum + a[i] > upper {
        return;
    }
    *rem -= 1;
    rec(i + 1, a, rem, cur_sum + a[i], upper);
    rec(i + 1, a, rem, cur_sum, upper);
}

fn can_k_of_less_or_equal(a: &Vec<i64>, k: usize, init_sum: i64, upper: i64) -> bool {
    let mut rem = k;
    if init_sum <= upper {
        rem -= 1;
    }
    rec(0, a, &mut rem, init_sum, upper);
    rem == 0
}

impl Solution {
    pub fn k_sum(a: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut a: Vec<_> = a.iter().map(|&x| -x as i64).collect();

        let sum_neg: i64 = a.iter().filter(|&&x| x < 0).sum();
        a = a.iter().map(|&x| x.abs()).collect();
        a.sort();

        let mut L = -i64::pow(10, 15);
        let mut R = i64::pow(10, 15);
        while L + 1 != R {
            let M = (L + R) / 2;
            if can_k_of_less_or_equal(&a, k, sum_neg, M) {
                R = M;
            } else {
                L = M;
            }
        }
        -R
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,4,-2], 5, 2)]
    #[case(vec![1,-2,3,4,-10,12], 16, 10)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::k_sum(nums, k);
        assert_eq!(actual, expected);
    }
}
