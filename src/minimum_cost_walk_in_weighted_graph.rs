//! Solution for https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph
//! 3108. Minimum Cost Walk in Weighted Graph

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
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        let mut dsu = DSU::new(n);
        let mut w = vec![-1 as i32; n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let weight = e[2];
            dsu.merge(a, b);
            w[a] &= weight;
            w[b] &= weight;
        }

        for v in 0..n {
            let p = dsu.get(v);
            w[p] &= w[v];
        }

        let mut ans = Vec::with_capacity(query.len());
        for q in &query {
            let a = q[0] as usize;
            let b = q[1] as usize;
            let p = dsu.get(a);
            if dsu.get(b) != p {
                ans.push(-1);
            } else {
                ans.push(w[p]);
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
    #[case(5, vec![vec![0,1,7],vec![1,3,7],vec![1,2,1]], vec![vec![0,3],vec![3,4]], vec![1,-1])]
    #[case(3, vec![vec![0,2,7],vec![0,1,15],vec![1,2,6],vec![1,2,1]], vec![vec![1,2]], vec![0])]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] query: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::minimum_cost(n, edges, query);
        assert_eq!(actual, expected);
    }
}
