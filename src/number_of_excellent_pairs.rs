//! Solution for https://leetcode.com/problems/number-of-excellent-pairs
//! 2354. Number of Excellent Pairs

const B: usize = 64;

impl Solution {
    pub fn count_excellent_pairs(mut a: Vec<i32>, k: i32) -> i64 {
        let n = a.len();
        let k = k as usize;

        a.sort();
        a.dedup();

        let mut cnt_with_pop = vec![0; B + 1];
        for &x in &a {
            let pop = x.count_ones() as usize;
            cnt_with_pop[pop] += 1;
        }
        let mut ans: i64 = 0;
        for left in 0..=B {
            for right in 0..=B {
                if left + right >= k {
                    ans += cnt_with_pop[left] as i64 * cnt_with_pop[right] as i64;
                }
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
    #[case(vec![1,2,3,1], 3, 5)]
    #[case(vec![5,1,1], 10, 0)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::count_excellent_pairs(nums, k);
        assert_eq!(actual, expected);
    }
}
