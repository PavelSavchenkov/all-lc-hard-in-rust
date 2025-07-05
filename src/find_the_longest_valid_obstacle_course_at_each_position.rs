//! Solution for https://leetcode.com/problems/find-the-longest-valid-obstacle-course-at-each-position
//! 1964. Find the Longest Valid Obstacle Course at Each Position

struct FenwTree {
    t: Vec<i32>,
}

impl FenwTree {
    fn new(n: usize) -> Self {
        FenwTree { t: vec![0; n] }
    }

    fn get_max(&self, mut r: usize) -> i32 {
        let mut ans = 0;
        loop {
            ans = ans.max(self.t[r]);
            r &= r + 1;
            if r == 0 {
                break;
            }
            r -= 1;
        }
        ans
    }

    fn upd(&mut self, mut i: usize, val: i32) {
        loop {
            self.t[i] = self.t[i].max(val);
            i |= i + 1;
            if i >= self.t.len() {
                break;
            }
        }
    }
}

impl Solution {
    pub fn longest_obstacle_course_at_each_position(a: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut vals = a.clone();
        vals.sort();
        vals.dedup();
        let mut arr = vec![0 as usize; n];
        for i in 0..n {
            let pos: usize = vals.binary_search(&a[i]).unwrap();
            arr[i] = pos;
        }
        // eprintln!("arr = {arr:#?}");
        let mut ans = vec![0 as i32; n];
        let mut t = FenwTree::new(vals.len());
        for i in 0..n {
            let cur_ans = t.get_max(arr[i]) + 1;
            ans[i] = cur_ans as i32;
            t.upd(arr[i], ans[i]);
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
    #[case(vec![1,2,3,2], vec![1,2,3,3])]
    #[case(vec![2,2,1], vec![1,2,1])]
    #[case(vec![3,1,5,6,4,2], vec![1,1,2,3,2,2])]
    fn case(#[case] obstacles: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::longest_obstacle_course_at_each_position(obstacles);
        assert_eq!(actual, expected);
    }
}
