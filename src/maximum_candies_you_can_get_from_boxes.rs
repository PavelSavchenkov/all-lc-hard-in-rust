//! Solution for https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes
//! 1298. Maximum Candies You Can Get from Boxes

use std::collections::BTreeSet;

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let n = status.len();

        let mut open_boxes = BTreeSet::new();
        let mut closed_boxes = BTreeSet::new();
        let mut keys_set = BTreeSet::new();
        for &i in &initial_boxes {
            let i = i as usize;
            if status[i] == 1 {
                open_boxes.insert(i);
            } else {
                closed_boxes.insert(i);
            }
        }

        let mut ans = 0;
        while !open_boxes.is_empty() {
            let i = open_boxes.pop_first().unwrap();
            for &key in &keys[i] {
                let key = key as usize;
                if closed_boxes.contains(&key) {
                    closed_boxes.remove(&key);
                    open_boxes.insert(key);
                } else {
                    keys_set.insert(key as usize);
                }
            }
            ans += candies[i];
            for &inside in &contained_boxes[i] {
                let inside = inside as usize;
                if status[inside] == 1 {
                    open_boxes.insert(inside);
                } else {
                    if keys_set.contains(&inside) {
                        keys_set.remove(&inside);
                        open_boxes.insert(inside);
                    } else {
                        closed_boxes.insert(inside);
                    }
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
    #[case(vec![1,0,1,0], vec![7,5,4,100], vec![vec![],vec![],vec![1],vec![]], vec![vec![1,2],vec![3],vec![],vec![]], vec![0], 16)]
    #[case(vec![1,0,0,0,0,0], vec![1,1,1,1,1,1], vec![vec![1,2,3,4,5],vec![],vec![],vec![],vec![],vec![]], vec![vec![1,2,3,4,5],vec![],vec![],vec![],vec![],vec![]], vec![0], 6)]
    fn case(
        #[case] status: Vec<i32>,
        #[case] candies: Vec<i32>,
        #[case] keys: Vec<Vec<i32>>,
        #[case] contained_boxes: Vec<Vec<i32>>,
        #[case] initial_boxes: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes);
        assert_eq!(actual, expected);
    }
}
