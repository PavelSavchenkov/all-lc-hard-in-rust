//! Solution for https://leetcode.com/problems/parallel-courses-ii
//! 1494. Parallel Courses II

fn remin(a: &mut usize, b: usize) {
    if *a > b {
        *a = b;
    }
}

impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        let mut prev_mask = vec![0; 1 << n];
        for rel in &relations {
            let from = (rel[0] - 1) as usize;
            let to = (rel[1] - 1) as usize;
            prev_mask[1 << to] |= 1 << from;
        }
        for mask in 1..((1 << n) as usize) {
            let b = mask.trailing_zeros();
            prev_mask[mask] = prev_mask[1 << b] | prev_mask[mask ^ (1 << b)];
        }

        let mut dp = vec![usize::MAX; 1 << n];
        dp[0] = 0;
        for mask in 1..1 << n {
            let mut sub = mask;
            while sub != 0 {
                let prev = prev_mask[sub];
                if sub.count_ones() <= k as u32 && (prev & (mask ^ sub)) == prev {
                    let prev_dp = dp[mask ^ sub];
                    if prev_dp < usize::MAX {
                        remin(&mut dp[mask], prev_dp + 1);
                    }
                }
                sub = (sub - 1) & mask;
            }
        }
        dp[(1 << n) - 1] as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, vec![vec![2,1],vec![3,1],vec![1,4]], 2, 3)]
    #[case(5, vec![vec![2,1],vec![3,1],vec![4,1],vec![1,5]], 2, 4)]
    fn case(
        #[case] n: i32,
        #[case] relations: Vec<Vec<i32>>,
        #[case] k: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::min_number_of_semesters(n, relations, k);
        assert_eq!(actual, expected);
    }
}
