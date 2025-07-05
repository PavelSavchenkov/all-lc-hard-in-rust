//! Solution for https://leetcode.com/problems/permutations-iv
//! 3470. Permutations IV

impl Solution {
    pub fn permute(n: i32, mut k: i64) -> Vec<i32> {
        let n = n as usize;

        let mut fact = vec![1; n + 1];
        for i in 2..fact.len() {
            fact[i] = i64::saturating_mul(i as i64, fact[i - 1]);
        }

        let mut ans = Vec::<usize>::new();
        let mut used = vec![false; n + 1];
        let mut cnt_par = vec![n / 2, n - n / 2];
        for i in 0..n {
            for x in 1..=n {
                if used[x] {
                    continue;
                }
                if !ans.is_empty() && (x % 2 == *ans.last().unwrap() % 2) {
                    continue;
                }
                cnt_par[x % 2] -= 1;
                let mut suff_cnt = 0;
                if cnt_par[0] == cnt_par[1] || cnt_par[1 - x % 2] == cnt_par[x % 2] + 1 {
                    suff_cnt = i64::saturating_mul(fact[cnt_par[0]], fact[cnt_par[1]]);
                }
                if suff_cnt < k {
                    k -= suff_cnt;
                } else {
                    ans.push(x);
                    used[x] = true;
                    break;
                }
                cnt_par[x % 2] += 1;
            }
            if ans.len() != i + 1 {
                return Vec::new();
            }
        }
        ans.iter().map(|&x| x as i32).collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, 6, vec![3,4,1,2])]
    #[case(3, 2, vec![3,2,1])]
    #[case(2, 3, vec![])]
    fn case(#[case] n: i32, #[case] k: i64, #[case] expected: Vec<i32>) {
        let actual = Solution::permute(n, k);
        assert_eq!(actual, expected);
    }
}
