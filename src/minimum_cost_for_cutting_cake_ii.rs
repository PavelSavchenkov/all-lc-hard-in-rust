//! Solution for https://leetcode.com/problems/minimum-cost-for-cutting-cake-ii
//! 3219. Minimum Cost for Cutting Cake II

#[derive(Default, Eq, PartialEq, Ord, PartialOrd)]
struct Coefs {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl Coefs {
    fn new() -> Self {
        Self::default()
    }

    fn merge(&self, other: &Self, i_shift: usize, j_shift: usize) -> Coefs {
        let mut rows = vec![0; self.rows.len().max(other.rows.len() + i_shift)];
        let mut cols = vec![0; self.cols.len().max(other.cols.len() + j_shift)];
        for i in 0..self.rows.len() {
            rows[i] += self.rows[i];
        }
        for i in 0..other.rows.len() {
            rows[i_shift + i] += other.rows[i];
        }
        for j in 0..self.cols.len() {
            cols[j] += self.cols[j];
        }
        for j in 0..other.cols.len() {
            cols[j_shift + j] += other.cols[j];
        }
        Self { rows, cols }
    }

    fn println(&self) {
        eprint!("rows: ");
        for c in &self.rows {
            eprint!("{} ", c);
        }
        eprint!("; ");
        eprint!("cols: ");
        for c in &self.cols {
            eprint!("{} ", c);
        }
        eprintln!();
    }
}

fn get_all_coefs(n: usize, m: usize) -> Vec<Coefs> {
    if n == 1 && m == 1 {
        return vec![Coefs::new()];
    }

    let mut ans = Vec::new();
    for i in 0..n - 1 {
        let rec_up = get_all_coefs(i + 1, m);
        let rec_down = get_all_coefs(n - i - 1, m);
        for c_up in &rec_up {
            for c_down in &rec_down {
                let mut cur = c_up.merge(c_down, i + 1, 0);
                cur.rows[i] += 1;
                ans.push(cur);
            }
        }
    }
    for j in 0..m - 1 {
        let rec_left = get_all_coefs(n, j + 1);
        let rec_right = get_all_coefs(n, m - j - 1);
        for c_left in &rec_left {
            for c_right in &rec_right {
                let mut cur = c_left.merge(c_right, 0, j + 1);
                cur.cols[j] += 1;
                ans.push(cur);
            }
        }
    }

    ans.sort();
    ans.dedup();

    ans
}

impl Solution {
    pub fn minimum_cost(n: i32, m: i32, mut hor: Vec<i32>, mut ver: Vec<i32>) -> i64 {
        // let coefs = get_all_coefs(3, 5);
        // for c in &coefs {
        //     c.println();
        // }
        // return 0;
        // didn't help

        ver.sort();
        ver.reverse();

        let mut suff = vec![0; ver.len() + 1];
        for i in (0..ver.len()).rev() {
            suff[i] = suff[i + 1] + ver[i] as i64;
        }

        if hor.is_empty() {
            return suff[0];
        }

        let m = m as usize;
        let n = n as usize;

        hor.sort();
        let H = *hor.last().unwrap() as usize;
        let mut cnt_hor = vec![0; H + 1];
        for &h in &hor {
            cnt_hor[h as usize] += 1;
        }
        hor.dedup();

        let mut ans = suff[0];
        for &h in &hor {
            let mut cur = i64::MAX;
            for k in 1..=m {
                let cand = h as i64 * k as i64 + suff[k - 1];
                cur = cur.min(cand);
            }
            ans += cur * cnt_hor[h as usize];
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
    #[case(3, 2, vec![1,3], vec![5], 13)]
    #[case(2, 2, vec![7], vec![4], 15)]
    fn case(
        #[case] m: i32,
        #[case] n: i32,
        #[case] horizontal_cut: Vec<i32>,
        #[case] vertical_cut: Vec<i32>,
        #[case] expected: i64,
    ) {
        let actual = Solution::minimum_cost(m, n, horizontal_cut, vertical_cut);
        assert_eq!(actual, expected);
    }
}
