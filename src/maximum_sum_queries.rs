//! Solution for https://leetcode.com/problems/maximum-sum-queries
//! 2736. Maximum Sum Queries

#[derive(Clone)]
struct FenwTreeMaxSuff {
    t: Vec<i64>,
}

impl FenwTreeMaxSuff {
    fn new(n: usize, val: i64) -> Self {
        Self { t: vec![val; n] }
    }

    fn relax_point(&mut self, pos: usize, val: i64) {
        assert!(pos < self.t.len());
        let mut i = self.t.len() - pos - 1;
        while i < self.t.len() {
            self.t[i] = self.t[i].max(val);
            i |= i + 1;
        }
    }

    // [r, t.len())
    fn get_max(&mut self, r: usize) -> i64 {
        let mut ans = i64::MIN;
        if r == self.t.len() {
            return ans;
        }
        let mut i = self.t.len() - r - 1;
        loop {
            ans = ans.max(self.t[i]);
            i &= i + 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        ans
    }
}

struct Item {
    x: i32,
    y: i32,
}

struct Query {
    x: i32,
    y: i32,
    i: usize,
}

impl Solution {
    pub fn maximum_sum_queries(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = nums1.len();
        let mut a = Vec::new();
        for i in 0..n {
            a.push(Item {
                x: nums1[i],
                y: nums2[i],
            });
        }
        a.sort_by_key(|e| e.x);

        let mut a_by_y: Vec<_> = (0..n).collect();
        a_by_y.sort_by_key(|&i| -a[i].y);

        let mut qs = Vec::new();
        for i in 0..queries.len() {
            let q = &queries[i];
            qs.push(Query {
                x: q[0],
                y: q[1],
                i,
            });
        }
        qs.sort_by_key(|q| -q.y);

        let mut tree = FenwTreeMaxSuff::new(n, -1);
        let mut ptr = 0;
        let mut answers = vec![-1; qs.len()];
        for q in &qs {
            while ptr < n && a[a_by_y[ptr]].y >= q.y {
                let i = a_by_y[ptr];
                tree.relax_point(i, (a[i].x + a[i].y) as i64);
                ptr += 1;
            }
            let i = a.partition_point(|e| e.x < q.x);
            answers[q.i] = tree.get_max(i).max(-1) as i32;
        }
        answers
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![4,3,1,2], vec![2,4,9,5], vec![vec![4,1],vec![1,3],vec![2,5]], vec![6,10,7])]
    #[case(vec![3,2,5], vec![2,3,4], vec![vec![4,4],vec![3,2],vec![1,1]], vec![9,9,9])]
    #[case(vec![2,1], vec![2,3], vec![vec![3,3]], vec![-1])]
    fn case(
        #[case] nums1: Vec<i32>,
        #[case] nums2: Vec<i32>,
        #[case] queries: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::maximum_sum_queries(nums1, nums2, queries);
        assert_eq!(actual, expected);
    }
}
