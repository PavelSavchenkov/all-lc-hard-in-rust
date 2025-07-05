//! Solution for https://leetcode.com/problems/reverse-pairs
//! 493. Reverse Pairs

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
    pub fn reverse_pairs(a: Vec<i32>) -> i32 {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as i64).collect();
        let mut vals = a.clone();
        vals.sort();
        vals.dedup();

        let mut tree = FenwTreeSum::new(vals.len());
        let mut ans = 0;
        for i in 0..n {
            let pos = vals.partition_point(|&x| x <= 2 * a[i]);
            let cnt = i as i64 - tree.get_sum(pos);
            ans += cnt;

            let pos = vals.binary_search(&a[i]).unwrap();
            tree.add_point(pos, 1);
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
    #[case(vec![1,3,2,3,1], 2)]
    #[case(vec![2,4,3,5,1], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::reverse_pairs(nums);
        assert_eq!(actual, expected);
    }
}
