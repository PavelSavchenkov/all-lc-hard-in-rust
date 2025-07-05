//! Solution for https://leetcode.com/problems/maximum-height-by-stacking-cuboids
//! 1691. Maximum Height by Stacking Cuboids

fn get_perm(mut p: usize) -> Vec<usize> {
    let mut was = vec![false; 3];
    let mut ans = Vec::with_capacity(3);
    let mut fact = vec![1; 3];
    for i in 1..fact.len() {
        fact[i] = fact[i - 1] * i;
    }
    for i in 0..3 {
        for j in 0..3 {
            if !was[j] {
                let up_to = fact[2 - i];
                if p < up_to {
                    ans.push(j);
                    was[j] = true;
                    break;
                } else {
                    p -= up_to;
                }
            }
        }
        assert!(ans.len() == i + 1);
    }
    ans
}

struct Cuboid {
    dims: Vec<i32>,
}

impl Cuboid {
    fn new(v: Vec<i32>) -> Self {
        assert!(v.len() == 3);
        Self { dims: v }
    }

    fn sum_dims(&self) -> i32 {
        self.dims.iter().sum()
    }

    // todo: precalc
    fn apply_perm(&self, p: usize) -> Self {
        let mut ndims = vec![0; 3];
        let perm = get_perm(p);
        for i in 0..3 {
            ndims[i] = self.dims[perm[i]];
        }
        Self::new(ndims)
    }

    fn fit_before(&self, other: &Self) -> bool {
        for i in 0..3 {
            if self.dims[i] < other.dims[i] {
                return false;
            }
        }
        true
    }
}

impl Solution {
    pub fn max_height(a: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let mut a: Vec<_> = a.iter().map(|v| Cuboid::new(v.clone())).collect();

        a.sort_by_cached_key(|c| c.sum_dims());
        a.reverse();

        let mut ans = 0;
        let mut dp = vec![vec![0; 6]; n];
        for i in 0..n {
            for perm in 0..6 {
                let cur_c = a[i].apply_perm(perm);
                let mut ndp = cur_c.dims[2];
                for j in 0..i {
                    for perm_j in 0..6 {
                        let prev_c = a[j].apply_perm(perm_j);
                        if prev_c.fit_before(&cur_c) {
                            ndp = ndp.max(dp[j][perm_j] + cur_c.dims[2]);
                        }
                    }
                }
                dp[i][perm] = ndp;
                ans = ans.max(ndp);
            }
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
    #[case(vec![vec![50,45,20],vec![95,37,53],vec![45,23,12]], 190)]
    #[case(vec![vec![38,25,45],vec![76,35,3]], 76)]
    #[case(vec![vec![7,11,17],vec![7,17,11],vec![11,7,17],vec![11,17,7],vec![17,7,11],vec![17,11,7]], 102)]
    fn case(#[case] cuboids: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::max_height(cuboids);
        assert_eq!(actual, expected);
    }
}
