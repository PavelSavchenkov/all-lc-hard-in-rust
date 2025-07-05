//! Solution for https://leetcode.com/problems/number-of-good-paths
//! 2421. Number of Good Paths

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
}

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = vals.len();

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }

        let mut by_vals: Vec<_> = (0..n).collect();
        by_vals.sort_by_key(|&v| vals[v]);

        let mut ans = 0;
        let mut dsu = DSU::new(n);
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && vals[by_vals[i]] == vals[by_vals[j]] {
                j += 1;
            }

            for k in i..j {
                let v = by_vals[k];
                for &to in &g[v] {
                    if vals[to] <= vals[v] {
                        dsu.merge(to, v);
                    }
                }
            }

            let mut roots = Vec::new();
            for k in i..j {
                let v = by_vals[k];
                roots.push(dsu.get(v));
            }

            roots.sort();
            let mut ii = 0;
            while ii < roots.len() {
                let mut jj = ii;
                while jj < roots.len() && roots[ii] == roots[jj] {
                    jj += 1;
                }
                let cnt = jj - ii;
                ans += cnt * (cnt - 1) / 2;
                ii = jj;
            }

            i = j;
        }

        ans += n;
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
    #[case(vec![1,3,2,1,3], vec![vec![0,1],vec![0,2],vec![2,3],vec![2,4]], 6)]
    #[case(vec![1,1,2,2,3], vec![vec![0,1],vec![1,2],vec![2,3],vec![2,4]], 7)]
    #[case(vec![1], vec![], 1)]
    fn case(#[case] vals: Vec<i32>, #[case] edges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::number_of_good_paths(vals, edges);
        assert_eq!(actual, expected);
    }
}
