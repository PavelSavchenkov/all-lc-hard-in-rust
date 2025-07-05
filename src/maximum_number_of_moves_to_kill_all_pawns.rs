//! Solution for https://leetcode.com/problems/maximum-number-of-moves-to-kill-all-pawns
//! 3283. Maximum Number of Moves to Kill All Pawns

use std::collections::VecDeque;

const N: usize = 50;

fn bit(mask: usize, b: usize) -> bool {
    ((mask >> b) & 1) == 1
}

impl Solution {
    pub fn max_moves(kx: i32, ky: i32, mut positions: Vec<Vec<i32>>) -> i32 {
        positions.push(vec![kx, ky]);
        let n = positions.len();
        let mut dist = vec![vec![0; n]; n];
        for i in 0..n {
            let x0 = positions[i][0] as usize;
            let y0 = positions[i][1] as usize;
            let mut d = vec![usize::MAX; N * N];
            d[x0 * N + y0] = 0;
            let mut q = VecDeque::new();
            q.push_back(x0 * N + y0);
            while !q.is_empty() {
                let v = q.pop_front().unwrap();
                let x = v / N;
                let y = v % N;
                for sw in 0..2 {
                    for dx_ in [-2, 2] {
                        for dy_ in [-1, 1] {
                            let mut dx = dx_;
                            let mut dy = dy_;
                            if sw == 1 {
                                std::mem::swap(&mut dx, &mut dy);
                            }
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if !(0 <= nx && nx < N as i32 && 0 <= ny && ny < N as i32) {
                                continue;
                            }
                            let nx = nx as usize;
                            let ny = ny as usize;
                            let u = nx * N + ny;
                            if d[u] > d[v] + 1 {
                                d[u] = d[v] + 1;
                                q.push_back(u);
                            }
                        }
                    }
                }
            }
            for j in 0..n {
                let x = positions[j][0] as usize;
                let y = positions[j][1] as usize;
                dist[i][j] = d[x * N + y];
            }
        }

        let n = n - 1;
        let mut dp = vec![vec![0; 1 << n]; n];
        for mask in 1..1 << n {
            for last in 0..n {
                if bit(mask, last) {
                    continue;
                }
                let mut mx = 0;
                let mut mn = usize::MAX;
                for next in 0..n {
                    if !bit(mask, next) {
                        continue;
                    }
                    let nmask = mask ^ (1 << next);
                    let ndp = dp[next][nmask] + dist[last][next];
                    mx = mx.max(ndp);
                    mn = mn.min(ndp);
                }
                assert!(mn < usize::MAX);
                if (n - mask.count_ones() as usize) % 2 == 0 {
                    // alice turn
                    dp[last][mask] = mx;
                } else {
                    dp[last][mask] = mn;
                }
            }
        }

        let mut ans = 0;
        for first in 0..n {
            let mask = ((1 << n) - 1) ^ (1 << first);
            let cur = dist[n][first] + dp[first][mask];
            ans = ans.max(cur);
        }
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
    #[case(1, 1, vec![vec![0,0]], 4)]
    #[case(0, 2, vec![vec![1,1],vec![2,2],vec![3,3]], 8)]
    #[case(0, 0, vec![vec![1,2],vec![2,4]], 3)]
    fn case(
        #[case] kx: i32,
        #[case] ky: i32,
        #[case] positions: Vec<Vec<i32>>,
        #[case] expected: i32,
    ) {
        let actual = Solution::max_moves(kx, ky, positions);
        assert_eq!(actual, expected);
    }
}
