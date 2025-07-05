//! Solution for https://leetcode.com/problems/swim-in-rising-water
//! 778. Swim in Rising Water

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
    pub fn swim_in_water(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let mut dsu = DSU::new(n * m);
        let mut ij = Vec::new();
        for i in 0..n {
            for j in 0..m {
                ij.push((i, j));
            }
        }
        ij.sort_by_key(|&(i, j)| g[i][j]);

        let src = 0;
        let dst = (n - 1) * m + (m - 1);
        for &(i, j) in &ij {
            for d in 0..4 {
                let ni = i as i32 + di[d];
                let nj = j as i32 + dj[d];
                if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if g[ni][nj] <= g[i][j] {
                    let v = i * m + j;
                    let u = ni * m + nj;
                    dsu.merge(v, u);
                }
            }

            if dsu.get(src) == dsu.get(dst) {
                return g[i][j];
            }
        }
        unreachable!()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,2],vec![1,3]], 3)]
    #[case(vec![vec![0,1,2,3,4],vec![24,23,22,21,5],vec![12,13,14,15,16],vec![11,17,18,19,20],vec![10,9,8,7,6]], 16)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::swim_in_water(grid);
        assert_eq!(actual, expected);
    }
}
