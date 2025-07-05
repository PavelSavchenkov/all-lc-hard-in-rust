//! Solution for https://leetcode.com/problems/checking-existence-of-edge-length-limited-paths
//! 1697. Checking Existence of Edge Length Limited Paths

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

struct Edge {
    a: usize,
    b: usize,
    w: i32,
    i: usize,
}

impl Edge {
    fn new(v: &Vec<i32>, i: usize) -> Self {
        Self {
            a: v[0] as usize,
            b: v[1] as usize,
            w: v[2],
            i,
        }
    }
}

impl Solution {
    pub fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = n as usize;

        let mut es: Vec<_> = (0..edge_list.len())
            .map(|i| Edge::new(&edge_list[i], i))
            .collect();
        let mut qs: Vec<_> = (0..queries.len())
            .map(|i| Edge::new(&queries[i], i))
            .collect();

        es.sort_by_key(|e| e.w);
        qs.sort_by_key(|e| e.w);

        let mut dsu = DSU::new(n);
        let mut ans = vec![false; qs.len()];
        let mut ptr_es = 0;
        for q in &qs {
            while ptr_es < es.len() && es[ptr_es].w < q.w {
                let e = &es[ptr_es];
                dsu.merge(e.a, e.b);
                ptr_es += 1;
            }
            ans[q.i] = dsu.get(q.a) == dsu.get(q.b);
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
    #[case(3, vec![vec![0,1,2],vec![1,2,4],vec![2,0,8],vec![1,0,16]], vec![vec![0,1,2],vec![0,2,5]], vec![false,true])]
    #[case(5, vec![vec![0,1,10],vec![1,2,5],vec![2,3,9],vec![3,4,13]], vec![vec![0,4,14],vec![1,4,13]], vec![true,false])]
    fn case(
        #[case] n: i32,
        #[case] edge_list: Vec<Vec<i32>>,
        #[case] queries: Vec<Vec<i32>>,
        #[case] expected: Vec<bool>,
    ) {
        let actual = Solution::distance_limited_paths_exist(n, edge_list, queries);
        assert_eq!(actual, expected);
    }
}
