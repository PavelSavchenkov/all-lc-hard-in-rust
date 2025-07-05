//! Solution for https://leetcode.com/problems/put-marbles-in-bags
//! 2551. Put Marbles in Bags

impl Solution {
    pub fn put_marbles(w: Vec<i32>, k: i32) -> i64 {
        let n = w.len();
        let k = k as usize;

        let mut vals = Vec::new();
        for i in 0..n - 1 {
            vals.push(w[i] + w[i + 1]);
        }
        vals.sort();

        let mut min: i64 = 0;
        for i in 0..k - 1 {
            min += vals[i] as i64;
        }
        let mut max: i64 = 0;
        vals.reverse();
        for i in 0..k - 1 {
            max += vals[i] as i64;
        }
        max - min
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,5,1], 2, 4)]
    #[case(vec![1,3], 2, 0)]
    fn case(#[case] weights: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::put_marbles(weights, k);
        assert_eq!(actual, expected);
    }
}
