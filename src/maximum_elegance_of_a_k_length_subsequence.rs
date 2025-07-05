//! Solution for https://leetcode.com/problems/maximum-elegance-of-a-k-length-subsequence
//! 2813. Maximum Elegance of a K-Length Subsequence

struct Item {
    profit: u32,
    category: usize,
}

impl Solution {
    pub fn find_maximum_elegance(items: Vec<Vec<i32>>, k: i32) -> i64 {
        let k = k as usize;
        let n = items.len();
        let mut items: Vec<_> = items
            .iter()
            .map(|v| Item {
                profit: v[0] as u32,
                category: (v[1] - 1) as usize,
            })
            .collect();
        items.sort_by_key(|item| item.profit);
        items.reverse();

        let mut sum_profit: u64 = 0;
        let mut cnt_distinct = 0;
        let mut seen = vec![false; n];
        let mut to_remove = Vec::new();
        for i in 0..k {
            sum_profit += items[i].profit as u64;
            if seen[items[i].category] {
                to_remove.push(items[i].profit);
            } else {
                seen[items[i].category] = true;
                cnt_distinct += 1;
            }
        }
        to_remove.reverse();
        let mut to_add = Vec::new();
        for i in k..n {
            if !seen[items[i].category] {
                to_add.push(items[i].profit);
                seen[items[i].category] = true;
            }
        }

        let mut ans = sum_profit + cnt_distinct as u64 * cnt_distinct as u64;
        for i in 0..to_remove.len() {
            if i >= to_add.len() {
                break;
            }
            sum_profit -= to_remove[i] as u64;
            sum_profit += to_add[i] as u64;
            cnt_distinct += 1;
            ans = ans.max(sum_profit + cnt_distinct as u64 * cnt_distinct as u64);
        }

        ans as i64
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![3,2],vec![5,1],vec![10,1]], 2, 17)]
    #[case(vec![vec![3,1],vec![3,1],vec![2,2],vec![5,3]], 3, 19)]
    #[case(vec![vec![1,1],vec![2,1],vec![3,1]], 3, 7)]
    fn case(#[case] items: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::find_maximum_elegance(items, k);
        assert_eq!(actual, expected);
    }
}
