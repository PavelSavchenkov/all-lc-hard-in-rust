//! Solution for https://leetcode.com/problems/count-almost-equal-pairs-ii
//! 3267. Count Almost Equal Pairs II

use std::collections::BTreeMap;

const P: usize = 7;

impl Solution {
    pub fn count_pairs(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut pw10 = vec![1; P];
        for i in 1..pw10.len() {
            pw10[i] = pw10[i - 1] * 10;
        }

        let mut ans = 0;
        let mut cnt_of = BTreeMap::new();
        for i in 0..n {
            let mut x = a[i];

            let mut ds = Vec::new();
            while x > 0 {
                ds.push(x % 10);
                x /= 10;
            }
            while ds.len() < P {
                ds.push(0);
            }

            let mut nums = Vec::new();
            for x1 in 0..ds.len() {
                for y1 in 0..x1 {
                    for x2 in 0..ds.len() {
                        for y2 in 0..ds.len() {
                            let mut cur_ds = ds.clone();
                            cur_ds.swap(x1, y1);
                            cur_ds.swap(x2, y2);
                            let mut num = 0;
                            for j in 0..cur_ds.len() {
                                num += cur_ds[j] * pw10[j];
                            }
                            nums.push(num);
                        }
                    }
                }
            }
            nums.sort();
            nums.dedup();
            for &num in &nums {
                ans += *cnt_of.entry(num).or_default();
            }

            *cnt_of.entry(a[i]).or_insert(0) += 1;
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
    #[case(vec![1023,2310,2130,213], 4)]
    #[case(vec![1,10,100], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::count_pairs(nums);
        assert_eq!(actual, expected);
    }
}
