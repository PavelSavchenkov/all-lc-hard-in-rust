//! Solution for https://leetcode.com/problems/sorted-gcd-pair-queries
//! 3312. Sorted GCD Pair Queries

impl Solution {
    pub fn gcd_values(a: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();

        let M = *a.iter().max().unwrap();

        let mut cnt_val = vec![0; M + 1];
        for &x in &a {
            cnt_val[x] += 1;
        }

        let mut cnt_with_g = vec![0; M + 1];
        for g in (1..=M).rev() {
            let mut cnt_div = 0;
            for x in (g..=M).step_by(g) {
                cnt_div += cnt_val[x];
            }
            let mut cur = cnt_div as i64 * (cnt_div - 1) as i64 / 2;
            for gg in (g + g..=M).step_by(g) {
                cur -= cnt_with_g[gg];
            }
            cnt_with_g[g] = cur;
        }

        let mut pref = vec![0; cnt_with_g.len() + 1];
        for i in 0..cnt_with_g.len() {
            pref[i + 1] = pref[i] + cnt_with_g[i];
        }

        let mut ans = Vec::new();
        for &q in &queries {
            let g = pref.partition_point(|&s| s <= q);
            ans.push((g - 1) as i32);
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
    #[case(vec![2,3,4], vec![0,2,2], vec![1,2,2])]
    #[case(vec![4,4,2,1], vec![5,3,1,0], vec![4,2,1,1])]
    #[case(vec![2,2], vec![0,0], vec![2,2])]
    fn case(#[case] nums: Vec<i32>, #[case] queries: Vec<i64>, #[case] expected: Vec<i32>) {
        let actual = Solution::gcd_values(nums, queries);
        assert_eq!(actual, expected);
    }
}
