//! Solution for https://leetcode.com/problems/count-subtrees-with-max-distance-between-cities
//! 1617. Count Subtrees With Max Distance Between Cities

impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        let mut nei = vec![0; n];
        for e in &edges {
            let a = (e[0] - 1) as usize;
            let b = (e[1] - 1) as usize;
            nei[a] |= 1 << b;
            nei[b] |= 1 << a;
        }

        let mut is_sub = vec![false; 1 << n];
        for i in 0..n {
            is_sub[1 << i] = true;
        }
        for mask in 1..((1 << n) as usize) {
            if is_sub[mask] {
                for i in 0..n {
                    if (nei[i] & mask) != 0 {
                        is_sub[mask | (1 << i)] = true;
                    }
                }
            }
        }

        let mut dist = vec![vec![n; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
            for j in 0..n {
                if ((nei[i] >> j) & 1) == 1 {
                    dist[i][j] = 1;
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }

        let mut ans = vec![0; n - 1];
        for mask in 1..((1 << n) as usize) {
            if is_sub[mask] && mask.count_ones() > 1 {
                let mut d = 0;
                for i in 0..n {
                    if ((mask >> i) & 1) == 1 {
                        for j in 0..n {
                            if ((mask >> j) & 1) == 1 {
                                d = d.max(dist[i][j]);
                            }
                        }
                    }
                }
                assert!(d > 0);
                ans[d - 1] += 1;
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
    #[case(4, vec![vec![1,2],vec![2,3],vec![2,4]], vec![3,4,0])]
    #[case(2, vec![vec![1,2]], vec![1])]
    #[case(3, vec![vec![1,2],vec![2,3]], vec![2,1])]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::count_subgraphs_for_each_diameter(n, edges);
        assert_eq!(actual, expected);
    }
}
