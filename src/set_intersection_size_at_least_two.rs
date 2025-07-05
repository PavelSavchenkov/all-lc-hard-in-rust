//! Solution for https://leetcode.com/problems/set-intersection-size-at-least-two
//! 757. Set Intersection Size At Least Two

#[derive(Copy, Clone)]
struct Seg {
    l: u32,
    r: u32,
}

impl Seg {
    fn new(v: &Vec<i32>) -> Self {
        Self {
            l: v[0] as u32,
            r: v[1] as u32,
        }
    }

    fn contain(&self, x: u32) -> bool {
        self.l <= x && x <= self.r
    }
}

impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut segs: Vec<_> = intervals.iter().map(|v| Seg::new(&v)).collect();
        let n = segs.len();

        segs.sort_by_key(|&s| s.r);

        let mut ans = Vec::new();
        let mut cnt = vec![0; n];
        for i in 0..n {
            while cnt[i] < 2 {
                let mut x = segs[i].r;
                if !ans.is_empty() && *ans.last().unwrap() == x {
                    x -= 1;
                    ans.insert(ans.len() - 1, x);
                } else {
                    ans.push(x);
                }
                for j in 0..n {
                    if segs[j].contain(x) {
                        cnt[j] += 1;
                    }
                }
            }
        }
        ans.len() as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3],vec![3,7],vec![8,9]], 5)]
    #[case(vec![vec![1,3],vec![1,4],vec![2,5],vec![3,5]], 3)]
    #[case(vec![vec![1,2],vec![2,3],vec![2,4],vec![4,5]], 5)]
    fn case(#[case] intervals: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::intersection_size_two(intervals);
        assert_eq!(actual, expected);
    }
}
