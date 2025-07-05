//! Solution for https://leetcode.com/problems/count-of-smaller-numbers-after-self
//! 315. Count of Smaller Numbers After Self

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
    pub fn count_smaller(a: Vec<i32>) -> Vec<i32> {
        let n = a.len();

        let mut vals = a.clone();
        vals.sort();
        vals.dedup();

        let mut tree = FenwTree::new(vals.len());
        let mut ans = vec![0; n];
        for i in (0..n).rev() {
            let pos = vals.binary_search(&a[i]).unwrap();
            ans[i] = tree.get_sum(pos) as i32;
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
    #[case(vec![5,2,6,1], vec![2,1,1,0])]
    #[case(vec![-1], vec![0])]
    #[case(vec![-1,-1], vec![0,0])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::count_smaller(nums);
        assert_eq!(actual, expected);
    }
}
