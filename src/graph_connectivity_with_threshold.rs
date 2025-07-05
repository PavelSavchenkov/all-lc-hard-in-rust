//! Solution for https://leetcode.com/problems/graph-connectivity-with-threshold
//! 1627. Graph Connectivity With Threshold

struct DSU {
    p: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self { p: vec![-1; n] }
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
        self.p[y] = x as i32;
        true
    }

    fn comp_size(&mut self, x: usize) -> usize {
        let root = self.get(x);
        (-self.p[root]) as usize
    }
}

impl Solution {
    pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let threshold = threshold as usize;

        let mut dsu = DSU::new(n + 1);
        for g in threshold + 1..=n {
            for x in (g..=n).step_by(g) {
                dsu.merge(g, x);
            }
        }

        let mut ans = Vec::new();
        for q in &queries {
            let a = q[0] as usize;
            let b = q[1] as usize;
            ans.push(dsu.get(a) == dsu.get(b));
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
    #[case(6, 2, vec![vec![1,4],vec![2,5],vec![3,6]], vec![false,false,true])]
    #[case(6, 0, vec![vec![4,5],vec![3,4],vec![3,2],vec![2,6],vec![1,3]], vec![true,true,true,true,true])]
    #[case(5, 1, vec![vec![4,5],vec![4,5],vec![3,2],vec![2,3],vec![3,4]], vec![false,false,false,false,false])]
    fn case(
        #[case] n: i32,
        #[case] threshold: i32,
        #[case] queries: Vec<Vec<i32>>,
        #[case] expected: Vec<bool>,
    ) {
        let actual = Solution::are_connected(n, threshold, queries);
        assert_eq!(actual, expected);
    }
}
