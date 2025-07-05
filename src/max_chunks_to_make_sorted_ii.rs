//! Solution for https://leetcode.com/problems/max-chunks-to-make-sorted-ii
//! 768. Max Chunks To Make Sorted II

impl Solution {
    pub fn max_chunks_to_sorted(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut vals = a.clone();
        vals.sort();

        let a: Vec<usize> = a.iter().map(|&x| vals.binary_search(&x).unwrap()).collect();
        let mut a_sorted = a.clone();
        a_sorted.sort();

        let mut cnt = vec![0; vals.len()];
        let mut update = |x: usize, diff: i32, cnt0: &mut usize| {
            if cnt[x] == 0 {
                *cnt0 -= 1;
            }
            cnt[x] += diff;
            if cnt[x] == 0 {
                *cnt0 += 1;
            }
        };
        let mut cnt0 = vals.len();
        let mut ans = 0;
        for i in 0..n {
            update(a[i], 1, &mut cnt0);
            update(a_sorted[i], -1, &mut cnt0);
            if cnt0 == n {
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
    #[case(vec![5,4,3,2,1], 1)]
    #[case(vec![2,1,3,4,4], 4)]
    fn case(#[case] arr: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_chunks_to_sorted(arr);
        assert_eq!(actual, expected);
    }
}
