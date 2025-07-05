//! Solution for https://leetcode.com/problems/count-the-number-of-good-partitions
//! 2963. Count the Number of Good Partitions

fn remax(a: &mut usize, b: usize) {
    if *a < b {
        *a = b;
    }
}

impl Solution {
    pub fn number_of_good_partitions(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut vals = a.clone();
        vals.sort();
        vals.dedup();

        let a: Vec<_> = a.iter().map(|x| vals.binary_search(&x).unwrap()).collect();

        let mut left = vec![usize::MAX; vals.len()];
        let mut right = vec![0; vals.len()];
        for i in 0..n {
            let x = a[i];
            left[x] = left[x].min(i);
            right[x] = right[x].max(i);
        }

        let mut segs = Vec::new();
        for x in 0..vals.len() {
            segs.push((left[x], right[x]));
        }
        segs.sort();

        let mut st = Vec::<(usize, usize)>::new();
        for &seg in &segs {
            if !st.is_empty() && st.last().unwrap().1 >= seg.0 {
                remax(&mut st.last_mut().unwrap().1, seg.1);
            } else {
                st.push(seg);
            }
        }

        let mut ans = 1;
        for it in 0..st.len() - 1 {
            ans = (ans * 2) % 1_000_000_007;
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
    #[case(vec![1,2,3,4], 8)]
    #[case(vec![1,1,1,1], 1)]
    #[case(vec![1,2,1,3], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::number_of_good_partitions(nums);
        assert_eq!(actual, expected);
    }
}
