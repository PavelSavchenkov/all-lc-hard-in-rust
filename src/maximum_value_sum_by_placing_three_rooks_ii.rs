//! Solution for https://leetcode.com/problems/maximum-value-sum-by-placing-three-rooks-ii
//! 3257. Maximum Value Sum by Placing Three Rooks II

impl Solution {
    pub fn maximum_value_sum(g: Vec<Vec<i32>>) -> i64 {
        let n = g.len();
        let m = g[0].len();

        let mut alive = vec![vec![false; m]; n];
        for i in 0..n {
            let mut js: Vec<_> = (0..m).collect();
            js.sort_by_key(|&j| -g[i][j]);
            js.truncate(3);
            for &j in &js {
                alive[i][j] = true;
            }
        }
        for j in 0..m {
            let mut is = Vec::new();
            for i in 0..n {
                if alive[i][j] {
                    is.push(i);
                    alive[i][j] = false;
                }
            }
            is.sort_by_key(|&i| -g[i][j]);
            is.truncate(3);
            for &i in &is {
                alive[i][j] = true;
            }
        }

        let mut ijs = Vec::new();
        for i in 0..n {
            for j in 0..m {
                if alive[i][j] {
                    ijs.push((i, j));
                }
            }
        }
        ijs.sort_by_key(|&(i, j)| -g[i][j]);

        let mut ans = i64::MIN;
        for a in 0..ijs.len() {
            for b in 0..a {
                let i1 = ijs[a].0;
                let j1 = ijs[a].1;
                let i2 = ijs[b].0;
                let j2 = ijs[b].1;
                if i1 == i2 || j1 == j2 {
                    continue;
                }
                let mut cur = g[i1][j1] as i64 + g[i2][j2] as i64;
                for c in 0..ijs.len() {
                    let i3 = ijs[c].0;
                    let j3 = ijs[c].1;
                    if i3 != i1 && i3 != i2 && j3 != j1 && j3 != j2 {
                        cur += g[i3][j3] as i64;
                        ans = ans.max(cur);
                        break;
                    }
                }
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
    #[case(vec![vec![-3,1,1,1],vec![-3,1,-3,1],vec![-3,2,1,1]], 4)]
    #[case(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]], 15)]
    #[case(vec![vec![1,1,1],vec![1,1,1],vec![1,1,1]], 3)]
    fn case(#[case] board: Vec<Vec<i32>>, #[case] expected: i64) {
        let actual = Solution::maximum_value_sum(board);
        assert_eq!(actual, expected);
    }
}
