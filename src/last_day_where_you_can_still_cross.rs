//! Solution for https://leetcode.com/problems/last-day-where-you-can-still-cross
//! 1970. Last Day Where You Can Still Cross

const di: [i32; 4] = [1, -1, 0, 0];
const dj: [i32; 4] = [0, 0, 1, -1];

impl Solution {
    pub fn latest_day_to_cross(n: i32, m: i32, cells: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = m as usize;

        let mut a = vec![vec![0 as usize; m]; n];
        for (t, v) in cells.iter().enumerate() {
            let i = v[0] as usize;
            let j = v[1] as usize;
            a[i - 1][j - 1] = t;
        }

        let mut L: usize = 0;
        let mut R: usize = n * m + 1;
        while L + 1 != R {
            let M = (L + R) / 2;

            let mut good = vec![vec![false; m]; n];
            let mut q = Vec::new();
            for j in 0..m {
                if a[0][j] > M {
                    good[0][j] = true;
                    q.push((0, j));
                }
            }
            let mut ql = 0;
            while ql < q.len() {
                let (i, j) = q[ql];
                ql += 1;
                assert!(good[i][j]);
                for d in 0..4 {
                    let ni = i as i32 + di[d];
                    let nj = j as i32 + dj[d];
                    if ni < 0 || ni >= n as i32 || nj < 0 || nj >= m as i32 {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if a[ni][nj] <= M || good[ni][nj] {
                        continue;
                    }
                    good[ni][nj] = true;
                    q.push((ni, nj));
                }
            }

            let ok = good[n - 1].iter().filter(|&&b| b).count() > 0;
            if ok {
                L = M;
            } else {
                R = M;
            }
        }

        (L + 1) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 2, vec![vec![1,1],vec![2,1],vec![1,2],vec![2,2]], 2)]
    #[case(2, 2, vec![vec![1,1],vec![1,2],vec![2,1],vec![2,2]], 1)]
    #[case(3, 3, vec![vec![1,2],vec![2,1],vec![3,3],vec![2,2],vec![1,1],vec![1,3],vec![2,3],vec![3,2],vec![3,1]], 3)]
    fn case(
        #[case] row: i32,
        #[case] col: i32,
        #[case] cells: Vec<Vec<i32>>,
        #[case] expected: i32,
    ) {
        let actual = Solution::latest_day_to_cross(row, col, cells);
        assert_eq!(actual, expected);
    }
}
