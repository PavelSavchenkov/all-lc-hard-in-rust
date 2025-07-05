//! Solution for https://leetcode.com/problems/minimum-time-to-complete-all-tasks
//! 2589. Minimum Time to Complete All Tasks

struct Task {
    start: u32,
    end: u32,
    duration: u32
}

impl Task {
    fn new(v: &Vec<i32>) -> Self {
        Self {
            start: v[0] as u32,
            end: v[1] as u32,
            duration: v[2] as u32,
        }
    }
}

impl Solution {
    pub fn find_minimum_time(tasks: Vec<Vec<i32>>) -> i32 {
        let mut tasks: Vec<_> = tasks.iter().map(|t| Task::new(&t)).collect();

        let mut ans = 0;
        let mut alive_tasks = tasks.len();
        let mut t = 1;
        while alive_tasks > 0 {
            let mut take = false;
            for task in &tasks {
                if task.duration > 0 && t == task.end - task.duration + 1 {
                    take = true;
                }
            }
            if !take {
                t += 1;
                continue
            }
            for task in &mut tasks {
                if task.duration > 0 && task.start <= t && t <= task.end {
                    task.duration -= 1;
                    if task.duration == 0 {
                        alive_tasks -= 1;
                    }
                }
            }
            ans += 1;
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
    #[case(vec![vec![2,3,1],vec![4,5,1],vec![1,5,2]], 2)]
    #[case(vec![vec![1,3,2],vec![2,5,3],vec![5,6,2]], 4)]
    fn case(#[case] tasks: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::find_minimum_time(tasks);
        assert_eq!(actual, expected);
    }
}
