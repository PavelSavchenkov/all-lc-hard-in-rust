//! Solution for https://leetcode.com/problems/build-a-matrix-with-conditions
//! 2392. Build a Matrix With Conditions

fn get_perm(cond: &Vec<Vec<i32>>, k: usize) -> Vec<usize> {
    let mut in_deg = vec![0; k];
    let mut g = vec![Vec::new(); k];
    for c in cond {
        let from = (c[0] - 1) as usize;
        let to = (c[1] - 1) as usize;
        g[from].push(to);
        in_deg[to] += 1;
    }

    let mut pos = vec![k; k];
    for it in 0..k {
        let mut next = k;
        for v in 0..k {
            if pos[v] == k && in_deg[v] == 0 {
                next = v;
                break;
            }
        }
        if next == k {
            return Vec::new();
        }
        pos[next] = it;
        for &to in &g[next] {
            in_deg[to] -= 1
        }
    }

    pos
}

impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let k = k as usize;

        let pos_row = get_perm(&row_conditions, k);
        let pos_col = get_perm(&col_conditions, k);

        if pos_row.is_empty() || pos_col.is_empty() {
            return Vec::new();
        }

        let mut ans = vec![vec![0; k]; k];
        for i in 0..k {
            ans[pos_row[i]][pos_col[i]] = (i + 1) as i32;
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
    #[case(3, vec![vec![1,2],vec![3,2]], vec![vec![2,1],vec![3,2]], vec![vec![3,0,0],vec![0,0,1],vec![0,2,0]])]
    #[case(3, vec![vec![1,2],vec![2,3],vec![3,1],vec![2,3]], vec![vec![2,1]], vec![])]
    fn case(
        #[case] k: i32,
        #[case] row_conditions: Vec<Vec<i32>>,
        #[case] col_conditions: Vec<Vec<i32>>,
        #[case] expected: Vec<Vec<i32>>,
    ) {
        let actual = Solution::build_matrix(k, row_conditions, col_conditions);
        assert_eq!(actual, expected);
    }
}
