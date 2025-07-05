//! Solution for https://leetcode.com/problems/maximum-segment-sum-after-removals
//! 2382. Maximum Segment Sum After Removals

struct DSU {
    p: Vec<i32>,
    sum: Vec<i64>,
}

impl DSU {
    fn new(n: usize, nums: &Vec<i32>) -> Self {
        Self {
            p: vec![-1; n],
            sum: nums.iter().map(|&x| x as i64).collect(),
        }
    }

    fn get(&mut self, x: usize) -> usize {
        if self.p[x] < 0 {
            x
        } else {
            self.p[x] = self.get(self.p[x] as usize) as i32;
            self.p[x] as usize
        }
    }

    fn merge(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.get(x);
        y = self.get(y);
        if x == y {
            return false;
        }
        if -self.p[x] < -self.p[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.p[x] += self.p[y];
        self.sum[x] += self.sum[y];
        self.p[y] = x as i32;
        true
    }
}

impl Solution {
    pub fn maximum_segment_sum(a: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
        let n = a.len();

        let mut ans = vec![0 as i64; n];
        let mut alive = vec![false; n];
        let mut dsu = DSU::new(n, &a);
        let mut mx: i64 = 0;
        for i in (0..n).rev() {
            let pos = remove_queries[i] as usize;
            alive[pos] = true;
            if pos > 0 && alive[pos - 1] {
                dsu.merge(pos - 1, pos);
            }
            if pos + 1 < n && alive[pos + 1] {
                dsu.merge(pos, pos + 1);
            }
            let comp = dsu.get(pos);
            let cur_sum = dsu.sum[comp];
            mx = mx.max(cur_sum);
            if i > 0 {
                ans[i - 1] = mx;
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
    #[case(vec![1,2,5,6,1], vec![0,3,2,4,1], vec![14,7,2,2,0])]
    #[case(vec![3,2,11,1], vec![3,2,1,0], vec![16,5,3,0])]
    fn case(#[case] nums: Vec<i32>, #[case] remove_queries: Vec<i32>, #[case] expected: Vec<i64>) {
        let actual = Solution::maximum_segment_sum(nums, remove_queries);
        assert_eq!(actual, expected);
    }
}
