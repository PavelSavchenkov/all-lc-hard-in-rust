//! Solution for https://leetcode.com/problems/course-schedule-iii
//! 630. Course Schedule III

struct Course {
    duration: usize,
    last_day: usize,
}

impl Course {
    fn new(v: &Vec<i32>) -> Self {
        Self {
            duration: v[0] as usize,
            last_day: v[1] as usize,
        }
    }
}

impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut cources: Vec<_> = courses.iter().map(|v| Course::new(&v)).collect();

        cources.sort_by_key(|c| c.last_day);

        let T = cources.last().unwrap().last_day + 2;

        let mut dp = vec![0; T];
        for c in &cources {
            if c.duration > c.last_day {
                continue;
            }
            for t in (1..=c.last_day - c.duration + 1).rev() {
                dp[t + c.duration] = dp[t + c.duration].max(dp[t] + 1);
            }
        }

        *dp.iter().max().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![100,200],vec![200,1300],vec![1000,1250],vec![2000,3200]], 3)]
    #[case(vec![vec![1,2]], 1)]
    #[case(vec![vec![3,2],vec![4,3]], 0)]
    fn case(#[case] courses: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::schedule_course(courses);
        assert_eq!(actual, expected);
    }
}
