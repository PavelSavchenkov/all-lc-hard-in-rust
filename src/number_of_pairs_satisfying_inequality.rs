//! Solution for https://leetcode.com/problems/number-of-pairs-satisfying-inequality
//! 2426. Number of Pairs Satisfying Inequality

struct FenwTreeSum {
    t: Vec<i64>,
}

impl FenwTreeSum {
    fn new(n: usize) -> Self {
        Self { t: vec![0; n] }
    }

    fn add_point(&mut self, pos: usize, val: i64) {
        let mut i = pos;
        while i < self.t.len() {
            self.t[i] += val;
            i |= i + 1;
        }
    }

    // [0; r)
    fn get_sum(&mut self, r: usize) -> i64 {
        if r == 0 {
            return 0;
        }
        let mut sum = 0;
        let mut i = r - 1;
        loop {
            sum += self.t[i];
            i &= i + 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        sum
    }
}

impl Solution {
    pub fn number_of_pairs(a: Vec<i32>, b: Vec<i32>, diff: i32) -> i64 {
        // a[i] - a[j] <= b[i] - b[j] + diff
        // a[i] - b[i] <= a[j] - b[j] + diff; i < j

        let n = a.len();

        let mut diffs = Vec::new();
        for i in 0..n {
            diffs.push(a[i] - b[i]);
        }
        diffs.sort();

        let mut ans = 0;
        let mut tree = FenwTreeSum::new(n);
        for j in 0..n {
            let upper = a[j] - b[j] + diff;
            let pos = diffs.partition_point(|&x| x <= upper);
            let cnt = tree.get_sum(pos);
            ans += cnt;

            let val = a[j] - b[j];
            let pos = diffs.binary_search(&val).unwrap();
            tree.add_point(pos, 1);
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
    #[case(vec![3,2,5], vec![2,2,1], 1, 3)]
    #[case(vec![3,-1], vec![-2,2], -1, 0)]
    fn case(
        #[case] nums1: Vec<i32>,
        #[case] nums2: Vec<i32>,
        #[case] diff: i32,
        #[case] expected: i64,
    ) {
        let actual = Solution::number_of_pairs(nums1, nums2, diff);
        assert_eq!(actual, expected);
    }
}
