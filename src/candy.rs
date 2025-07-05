//! Solution for https://leetcode.com/problems/candy
//! 135. Candy

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();

        let mut order: Vec<_> = (0..n).collect();
        order.sort_by_key(|&i| ratings[i]);

        let mut vals = vec![-1; n];
        for &i in order.iter() {
            vals[i] = 1;
            if i > 0 && ratings[i] > ratings[i - 1] {
                vals[i] = vals[i].max(vals[i - 1] + 1);
            }
            if i + 1 < n && ratings[i] > ratings[i + 1] {
                vals[i] = vals[i].max(vals[i + 1] + 1);
            }
        }

        vals.iter().sum()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,0,2], 5)]
    #[case(vec![1,2,2], 4)]
    fn case(#[case] ratings: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::candy(ratings);
        assert_eq!(actual, expected);
    }
}
