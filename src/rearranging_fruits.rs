//! Solution for https://leetcode.com/problems/rearranging-fruits
//! 2561. Rearranging Fruits

use std::collections::HashMap;

fn get_giveup_array(a: &Vec<i32>, mut want_cnt: HashMap<i32, usize>) -> Vec<i32> {
    let mut ans = Vec::new();
    for &x in a {
        let cnt_ptr = want_cnt.get_mut(&x).unwrap();
        if *cnt_ptr == 0 {
            ans.push(x);
        } else {
            *cnt_ptr -= 1;
        }
    }
    ans
}

impl Solution {
    pub fn min_cost(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let n = a.len();
        assert!(n == b.len());

        let mut cnt = HashMap::new();
        for i in 0..n {
            for x in [a[i], b[i]] {
                *cnt.entry(x).or_insert(0) += 1;
            }
        }

        for (k, v) in &mut cnt {
            if *v % 2 != 0 {
                return -1;
            }
            *v /= 2;
        }

        let global_min = *a.iter().min().unwrap().min(b.iter().min().unwrap());

        let a = get_giveup_array(&a, cnt.clone());
        let b = get_giveup_array(&b, cnt.clone());
        assert!(a.len() == b.len());

        let mut all = a.clone();
        all.extend_from_slice(&b);

        all.sort();
        all.truncate(a.len());

        let mut ans: i64 = 0;
        for &x in &all {
            ans += (x as i64).min(global_min as i64 * 2);
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
    #[case(vec![4,2,2,2], vec![1,4,1,2], 1)]
    #[case(vec![2,3,4,1], vec![3,2,5,1], -1)]
    fn case(#[case] basket1: Vec<i32>, #[case] basket2: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::min_cost(basket1, basket2);
        assert_eq!(actual, expected);
    }
}
