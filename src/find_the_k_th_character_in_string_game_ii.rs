//! Solution for https://leetcode.com/problems/find-the-k-th-character-in-string-game-ii
//! 3307. Find the K-th Character in String Game II

impl Solution {
    pub fn kth_character(mut k: i64, operations: Vec<i32>) -> char {
        let mut i = 0;
        while 2 * (1 << i) < k {
            i += 1;
        }
        assert!(i < operations.len());

        let mut shift = 0;
        loop {
            while i > 0 && 2 * (1 << (i - 1)) >= k {
                i -= 1;
            }
            if k == 1 {
                break;
            }
            assert!(k > (1 << i));
            if operations[i] == 1 {
                shift += 1;
            }
            k -= 1 << i;
        }

        let ans = b'a' + (shift % 26) as u8;
        ans as char
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![0,0,0], 'a')]
    #[case(10, vec![0,1,0,1], 'b')]
    fn case(#[case] k: i64, #[case] operations: Vec<i32>, #[case] expected: char) {
        let actual = Solution::kth_character(k, operations);
        assert_eq!(actual, expected);
    }
}
