//! Solution for https://leetcode.com/problems/maximum-building-height
//! 1840. Maximum Building Height

struct Restriction {
    id: i32,
    max: i32,
}

impl Restriction {
    fn new(v: &Vec<i32>) -> Self {
        Self {
            id: v[0],
            max: v[1],
        }
    }
}

impl Solution {
    pub fn max_building(n: i32, mut restrictions: Vec<Vec<i32>>) -> i32 {
        restrictions.push(vec![1, 0]);
        let mut rs: Vec<_> = restrictions.iter().map(|v| Restriction::new(&v)).collect();
        rs.sort_by_key(|r| r.id);
        drop(restrictions);

        let can = |h: i32| -> bool {
            let mut suff_min = vec![n + 1; rs.len() + 1];
            for i in (0..rs.len()).rev() {
                let j = rs[i].id;
                let max = rs[i].max;
                suff_min[i] = suff_min[i + 1].min(max - h + j);
            }
            let mut lower = -1;
            for i in 0..rs.len() {
                let j = rs[i].id;
                let max = rs[i].max;
                lower = lower.max(h - max + j);
                let upper = suff_min[i + 1];
                if lower <= upper && lower <= n {
                    return true;
                }
            }
            false
        };

        let mut L = -1;
        let mut R = n + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if can(M) {
                L = M;
            } else {
                R = M;
            }
        }
        L
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![vec![2,1],vec![4,1]], 2)]
    #[case(6, vec![], 5)]
    #[case(10, vec![vec![5,3],vec![2,5],vec![7,4],vec![10,3]], 5)]
    fn case(#[case] n: i32, #[case] restrictions: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::max_building(n, restrictions);
        assert_eq!(actual, expected);
    }
}
