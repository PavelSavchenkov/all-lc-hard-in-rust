//! Solution for https://leetcode.com/problems/first-missing-positive
//! 41. First Missing Positive

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        nums = nums.iter().map(|x| x - 1).collect();
        // eprintln!("{nums:#?}");
        let n = nums.len();
        let mut cnt_good = 0;
        for i in 0..n {
            if nums[i] == i as i32 {
                cnt_good += 1;
            }
        }
        for i in 0..n {
            loop {
                let x = nums[i];
                if x < 0 || x >= n as i32 {
                    break;
                }
                let x = x as usize;
                if x == i {
                    break;
                }
                if nums[x] == x as i32 {
                    break;
                }
                // eprintln!("swap {i} {x}");
                let old_cnt_good = cnt_good;
                for idx in [i, x] {
                    if nums[idx] == idx as i32 {
                        cnt_good -= 1;
                    }
                }
                nums.swap(i, x);
                for idx in [i, x] {
                    if nums[idx] == idx as i32 {
                        cnt_good += 1;
                    }
                }
                assert!(old_cnt_good < cnt_good);
            }
        }
        // eprintln!("{nums:#?}");
        for i in 0..n {
            if nums[i] != i as i32 {
                return (i + 1) as i32;
            }
        }
        (n + 1) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,0], 3)]
    #[case(vec![3,4,-1,1], 2)]
    #[case(vec![7,8,9,11,12], 1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::first_missing_positive(nums);
        assert_eq!(actual, expected);
    }
}
