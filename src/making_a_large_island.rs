//! Solution for https://leetcode.com/problems/making-a-large-island
//! 827. Making A Large Island

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

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn largest_island(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let mut dsu = DSU::new(n * m);
        for i in 0..n {
            for j in 0..m {
                if g[i][j] != 1 {
                    continue;
                }
                for d in 0..4 {
                    let ni = i as i32 + di[d];
                    let nj = j as i32 + dj[d];
                    if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if g[ni][nj] == 1 {
                        dsu.merge(i * m + j, ni * m + nj);
                    }
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if g[i][j] == 1 {
                    let root = dsu.get(i * m + j);
                    ans = ans.max(-dsu.p[root]);
                    continue;
                }
                let mut nei = Vec::new();
                for d in 0..4 {
                    let ni = i as i32 + di[d];
                    let nj = j as i32 + dj[d];
                    if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if g[ni][nj] == 1 {
                        let root = dsu.get(ni * m + nj);
                        nei.push((root, -dsu.p[root]));
                    }
                }
                nei.sort();
                nei.dedup();
                let mut cur = 0;
                for (root, size) in &nei {
                    cur += size;
                }
                ans = ans.max(cur + 1);
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
    #[case(vec![vec![1,0],vec![0,1]], 3)]
    #[case(vec![vec![1,1],vec![1,0]], 4)]
    #[case(vec![vec![1,1],vec![1,1]], 4)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::largest_island(grid);
        assert_eq!(actual, expected);
    }
}
