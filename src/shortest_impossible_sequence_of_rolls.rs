//! Solution for https://leetcode.com/problems/shortest-impossible-sequence-of-rolls
//! 2350. Shortest Impossible Sequence of Rolls

impl Solution {
    pub fn shortest_sequence(a: Vec<i32>, k: i32) -> i32 {
        let a: Vec<_> = a.iter().map(|&x| (x - 1) as usize).collect();
        let n = a.len();
        let k = k as usize;

        let mut prev = vec![Option::<usize>::None; k];
        let mut dp = vec![usize::MAX; n];
        let mut st = Vec::new();
        for i in 0..n {
            let x = a[i];
            if prev[x].is_none() {
                dp[i] = 1;
            } else {
                let l = prev[x].unwrap();
                let pos = st.partition_point(|&j| j < l);
                assert!(pos < st.len());
                assert!(st[pos] < i);
                dp[i] = dp[st[pos]] + 1;
            }

            while !st.is_empty() && dp[*st.last().unwrap()] >= dp[i] {
                st.pop();
            }
            st.push(i);

            prev[x] = Some(i);
        }

        for v in 0..k {
            if prev[v].is_none() {
                return 1;
            }
        }

        let mut cnt_seen = 0;
        let mut seen = vec![false; k];
        let mut i_has_all = n;
        for i in (0..n).rev() {
            if !seen[a[i]] {
                seen[a[i]] = true;
                cnt_seen += 1;
                if cnt_seen == k {
                    i_has_all = i;
                    break;
                }
            }
        }
        assert!(i_has_all < n);

        let mut ans = usize::MAX;
        for i in 0..n {
            if i + 1 > i_has_all {
                ans = ans.min(dp[i] + 1);
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
    #[case(vec![4,2,1,2,3,3,2,4,1], 4, 3)]
    #[case(vec![1,1,2,2], 2, 2)]
    #[case(vec![1,1,3,2,2,2,3,3], 4, 1)]
    fn case(#[case] rolls: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::shortest_sequence(rolls, k);
        assert_eq!(actual, expected);
    }
}
