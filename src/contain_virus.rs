//! Solution for https://leetcode.com/problems/contain-virus
//! 749. Contain Virus

#[derive(Eq, PartialEq, Copy, Clone)]
enum DfsType {
    Count,
    BuildWalls,
    Spread,
}

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

struct Grid {
    infected: Vec<Vec<bool>>,
    blocked: Vec<Vec<Vec<bool>>>,
    n: usize,
    m: usize,
}

impl Grid {
    fn new(infected: Vec<Vec<i32>>) -> Self {
        let n = infected.len();
        let m = infected[0].len();
        let mut this = Self {
            infected: vec![vec![false; m]; n],
            blocked: vec![vec![vec![false; 4]; m]; n],
            n,
            m,
        };
        for i in 0..n {
            for j in 0..m {
                if infected[i][j] == 1 {
                    this.infected[i][j] = true;
                }
            }
        }
        this
    }

    fn step(&mut self) -> bool {
        let mut last_visited = vec![vec![0; self.m]; self.n]; // for not infected cells
        let mut is_visited = vec![vec![false; self.m]; self.n]; // for infected cells
        let mut timer = 0;
        // find max component
        let mut max_cnt = 0;
        let mut max_start = (0, 0);
        for i in 0..self.n {
            for j in 0..self.m {
                if !self.infected[i][j] || is_visited[i][j] {
                    continue;
                }
                timer += 1;
                let cnt = self.dfs(
                    i,
                    j,
                    &mut last_visited,
                    &mut is_visited,
                    DfsType::Count,
                    &mut timer,
                );
                if cnt > max_cnt {
                    max_cnt = cnt;
                    max_start = (i, j);
                }
            }
        }

        if max_cnt == 0 {
            return false;
        }

        // build walls
        is_visited = vec![vec![false; self.m]; self.n];
        self.dfs(
            max_start.0,
            max_start.1,
            &mut last_visited,
            &mut is_visited,
            DfsType::BuildWalls,
            &mut timer,
        );

        // spread the virus
        timer += 1;
        for i in 0..self.n {
            for j in 0..self.m {
                if !self.infected[i][j] || is_visited[i][j] {
                    continue;
                }
                timer += 1;
                self.dfs(
                    i,
                    j,
                    &mut last_visited,
                    &mut is_visited,
                    DfsType::Spread,
                    &mut timer,
                );
            }
        }

        true
    }

    fn dfs(
        &mut self,
        i: usize,
        j: usize,
        last_visited: &mut Vec<Vec<u32>>,
        is_visited: &mut Vec<Vec<bool>>,
        dfs_type: DfsType,
        timer: &mut u32,
    ) -> usize {
        assert!(self.infected[i][j]);
        assert!(!is_visited[i][j]);
        is_visited[i][j] = true;
        let mut cnt = 0;
        for d in 0..4 {
            let next = self.try_go(i, j, d);
            if next.is_none() {
                continue;
            }
            let (ni, nj) = next.unwrap();
            if !self.infected[ni][nj] {
                if last_visited[ni][nj] != *timer {
                    cnt += 1;
                    if dfs_type == DfsType::Spread {
                        self.infected[ni][nj] = true;
                        is_visited[ni][nj] = true;
                    }
                    last_visited[ni][nj] = *timer;
                }
                if dfs_type == DfsType::BuildWalls {
                    self.blocked[i][j][d] = true;
                    self.blocked[ni][nj][d ^ 1] = true;
                }
            } else {
                if !is_visited[ni][nj] {
                    cnt += self.dfs(ni, nj, last_visited, is_visited, dfs_type, timer);
                }
            }
        }
        cnt
    }

    // true if next cell is inside the grid and not blocked by a wall
    fn try_go(&self, i: usize, j: usize, d: usize) -> Option<(usize, usize)> {
        if self.blocked[i][j][d] {
            return None;
        }
        let ni = i as i32 + di[d];
        let nj = j as i32 + dj[d];
        if !(0 <= ni && ni < self.n as i32 && 0 <= nj && nj < self.m as i32) {
            return None;
        }
        let ni = ni as usize;
        let nj = nj as usize;
        Some((ni, nj))
    }

    fn cnt_walls(&self) -> usize {
        let mut ans = 0;
        for i in 0..self.n {
            for j in 0..self.m {
                for d in 0..4 {
                    if self.blocked[i][j][d] {
                        ans += 1;
                    }
                }
            }
        }
        assert!(ans % 2 == 0);
        ans / 2
    }
}

impl Solution {
    pub fn contain_virus(is_infected: Vec<Vec<i32>>) -> i32 {
        let mut g = Grid::new(is_infected);

        while g.step() {}

        g.cnt_walls() as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1,0,0,0,0,0,1],vec![0,1,0,0,0,0,0,1],vec![0,0,0,0,0,0,0,1],vec![0,0,0,0,0,0,0,0]], 10)]
    #[case(vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]], 4)]
    #[case(vec![vec![1,1,1,0,0,0,0,0,0],vec![1,0,1,0,1,1,1,1,1],vec![1,1,1,0,0,0,0,0,0]], 13)]
    fn case(#[case] is_infected: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::contain_virus(is_infected);
        assert_eq!(actual, expected);
    }
}
