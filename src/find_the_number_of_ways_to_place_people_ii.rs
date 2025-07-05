//! Solution for https://leetcode.com/problems/find-the-number-of-ways-to-place-people-ii
//! 3027. Find the Number of Ways to Place People II

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        points.sort_by_key(|p| (p[0], -p[1]));

        let mut ans = 0;
        for i in 0..n {
            let x0 = points[i][0];
            let y0 = points[i][1];
            let mut max_y = i32::MIN;
            for j in i + 1..n {
                let y = points[j][1];
                if y > y0 {
                    continue;
                }
                let x = points[j][0];
                assert!(x >= x0);
                if y > max_y {
                    ans += 1;
                    max_y = y;
                }
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
    #[case(vec![vec![1,1],vec![2,2],vec![3,3]], 0)]
    #[case(vec![vec![6,2],vec![4,4],vec![2,6]], 2)]
    #[case(vec![vec![3,1],vec![1,3],vec![1,1]], 2)]
    fn case(#[case] points: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::number_of_pairs(points);
        assert_eq!(actual, expected);
    }
}
