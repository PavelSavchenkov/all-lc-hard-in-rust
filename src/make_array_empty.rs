//! Solution for https://leetcode.com/problems/make-array-empty
//! 2659. Make Array Empty

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
    fn get_sum_pref(&self, r: usize) -> i64 {
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

    fn get_sum(&self, l: usize, r: usize) -> i64 {
        self.get_sum_pref(r) - self.get_sum_pref(l)
    }
}

impl Solution {
    pub fn count_operations_to_empty_array(a: Vec<i32>) -> i64 {
        let n = a.len();

        let mut vals = a.clone();
        vals.sort();
        vals.dedup();
        assert!(vals.len() == n);

        let mut pos = vec![0; n];
        for i in 0..n {
            let ord_i = vals.binary_search(&a[i]).unwrap();
            pos[ord_i] = i;
        }

        let mut tree = FenwTreeSum::new(n);
        for i in 0..n {
            tree.add_point(i, 1);
        }

        let mut ans: i64 = 0;
        let mut ptr = 0;
        for val in 0..n {
            let next = pos[val];
            let between = if ptr < next {
                tree.get_sum(ptr, next)
            } else if ptr > next {
                tree.get_sum(ptr, n) + tree.get_sum(0, next)
            } else {
                0
            };
            ans += between;
            ans += 1;
            tree.add_point(next, -1);
            ptr = next;
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
    #[case(vec![3,4,-1], 5)]
    #[case(vec![1,2,4,3], 5)]
    #[case(vec![1,2,3], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::count_operations_to_empty_array(nums);
        assert_eq!(actual, expected);
    }
}
