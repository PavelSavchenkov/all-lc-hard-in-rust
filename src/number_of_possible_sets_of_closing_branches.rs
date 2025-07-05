//! Solution for https://leetcode.com/problems/number-of-possible-sets-of-closing-branches
//! 2959. Number of Possible Sets of Closing Branches

fn bit(mask: usize, b: usize) -> bool {
    ((mask >> b) & 1) == 1
}

impl Solution {
    pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut g = vec![vec![i32::MAX; n]; n];
        for r in &roads {
            let a = r[0] as usize;
            let b = r[1] as usize;
            let w = r[2];
            g[a][b] = g[a][b].min(w);
            g[b][a] = g[b][a].min(w);
        }

        let mut ans = 0;
        for mask in 0..1 << n {
            let mut gg = g.clone();
            for a in 0..n {
                for b in 0..n {
                    if bit(mask, a) || bit(mask, b) {
                        gg[a][b] = i32::MAX;
                    }
                }
            }
            for a in 0..n {
                gg[a][a] = 0;
            }

            for k in 0..n {
                for i in 0..n {
                    for j in 0..n {
                        let left = gg[i][k];
                        let right = gg[k][j];
                        if left < i32::MAX && right < i32::MAX {
                            gg[i][j] = gg[i][j].min(left + right);
                        }
                    }
                }
            }

            let mut ok = true;
            for a in 0..n {
                for b in 0..n {
                    if !bit(mask, a) && !bit(mask, b) && gg[a][b] > max_distance {
                        ok = false;
                    }
                }
            }
            if ok {
                ans += 1;
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
    #[case(3, 5, vec![vec![0,1,2],vec![1,2,10],vec![0,2,10]], 5)]
    #[case(3, 5, vec![vec![0,1,20],vec![0,1,10],vec![1,2,2],vec![0,2,2]], 7)]
    #[case(1, 10, vec![], 2)]
    fn case(
        #[case] n: i32,
        #[case] max_distance: i32,
        #[case] roads: Vec<Vec<i32>>,
        #[case] expected: i32,
    ) {
        let actual = Solution::number_of_sets(n, max_distance, roads);
        assert_eq!(actual, expected);
    }
}
