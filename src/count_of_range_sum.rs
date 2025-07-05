//! Solution for https://leetcode.com/problems/count-of-range-sum
//! 327. Count of Range Sum

struct FenwTree {
    t: Vec<i64>,
}

impl FenwTree {
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
    pub fn count_range_sum(a: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n = a.len();
        let lower = lower as i64;
        let upper = upper as i64;

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + a[i] as i64;
        }

        let mut vals = pref.clone();
        vals.sort();
        // vals.dedup();

        let mut tree_sum = FenwTree::new(vals.len());

        let mut ans: i64 = 0;
        for i in 0..n {
            {
                let pos = vals.binary_search(&pref[i]).unwrap();
                tree_sum.add_point(pos, 1);
            }

            // [x, i] segments
            // x = 0...i
            // pref[i + 1] - pref[x] <= upper ---> pref[x] >= pref[i + 1] - upper
            // pref[i + 1] - pref[x] >= lower ---> pref[x] <= pref[i + 1] - lower
            let L = pref[i + 1] - upper;
            let R = pref[i + 1] - lower;

            let l = vals.partition_point(|&v| v < L);
            let r = vals.partition_point(|&v| v <= R);
            // [l, r)
            let cnt = tree_sum.get_sum(r) - tree_sum.get_sum(l);
            ans += cnt;
        }

        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![-2,5,-1], -2, 2, 3)]
    #[case(vec![0], 0, 0, 1)]
    fn case(#[case] nums: Vec<i32>, #[case] lower: i32, #[case] upper: i32, #[case] expected: i32) {
        let actual = Solution::count_range_sum(nums, lower, upper);
        assert_eq!(actual, expected);
    }
}
