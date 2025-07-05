//! Solution for https://leetcode.com/problems/count-good-triplets-in-an-array
//! 2179. Count Good Triplets in an Array

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

fn count_left(a: &Vec<usize>, b: &Vec<usize>) -> Vec<i64> {
    let n = a.len();

    let mut pos_b = vec![0; n];
    for i in 0..n {
        pos_b[b[i]] = i;
    }

    let mut tree = FenwTree::new(n);

    let mut cnt = vec![0; n];
    for i in 0..n {
        let y = a[i];
        cnt[i] = tree.get_sum(pos_b[y]);
        tree.add_point(pos_b[y], 1);
    }

    cnt
}

impl Solution {
    pub fn good_triplets(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let mut a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let mut b: Vec<_> = b.iter().map(|&x| x as usize).collect();
        let n = a.len();

        let left = count_left(&a, &b);

        a.reverse();
        b.reverse();
        let mut right = count_left(&a, &b);
        right.reverse();
        a.reverse();
        b.reverse();

        let mut ans: i64 = 0;
        for i in 0..n {
            ans += left[i] * right[i];
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
    #[case(vec![2,0,1,3], vec![0,1,2,3], 1)]
    #[case(vec![4,0,1,3,2], vec![4,1,0,2,3], 4)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::good_triplets(nums1, nums2);
        assert_eq!(actual, expected);
    }
}
