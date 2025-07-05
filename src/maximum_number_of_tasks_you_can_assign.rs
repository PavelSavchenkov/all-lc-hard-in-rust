//! Solution for https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign
//! 2071. Maximum Number of Tasks You Can Assign

impl Solution {
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        let m = tasks.len();
        let n = workers.len();
        let pills = pills as usize;

        tasks.sort();
        tasks.reverse();

        workers.sort();
        workers.reverse();

        // O(N) per check, O(N log N) overall
        let can_assign = |cnt: usize| -> bool {
            if cnt == 0 {
                return true;
            }
            let tasks = tasks[m - cnt..m].to_vec();
            let workers = workers[0..cnt].to_vec();
            let mut used_pills = 0;
            let mut next = Vec::with_capacity(cnt);
            let mut prev = Vec::with_capacity(cnt);
            for i in 0..cnt {
                if i + 1 == cnt {
                    next.push(None)
                } else {
                    next.push(Some(i + 1));
                }
                if i == 0 {
                    prev.push(None)
                } else {
                    prev.push(Some(i - 1))
                }
            }
            let mut head = Some(0);
            let mut ptr = Some(0);
            for &task in &tasks {
                if workers[head.unwrap()] >= task {
                    let new_head = next[head.unwrap()];
                    if !new_head.is_none() {
                        prev[new_head.unwrap()] = None;
                    }
                    head = new_head;
                    continue;
                }
                if used_pills == pills {
                    return false;
                }
                if ptr.unwrap() < head.unwrap() {
                    ptr = head;
                }
                while !next[ptr.unwrap()].is_none()
                    && workers[next[ptr.unwrap()].unwrap()] + strength >= task
                {
                    ptr = next[ptr.unwrap()];
                }
                if ptr.is_none() || workers[ptr.unwrap()] + strength < task {
                    return false;
                }
                used_pills += 1;
                let left = prev[ptr.unwrap()];
                let right = next[ptr.unwrap()];
                if left.is_none() {
                    assert!(ptr == head);
                    if right.is_none() {
                        return true;
                    }
                    prev[right.unwrap()] = None;
                    head = right;
                    ptr = right;
                    continue;
                }
                next[left.unwrap()] = right;
                if !right.is_none() {
                    prev[right.unwrap()] = left;
                }
                ptr = left;
            }
            true
        };

        let mut L = 0;
        let mut R = m.min(n) + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if can_assign(M) {
                L = M
            } else {
                R = M
            }
        }

        L as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,2,1], vec![0,3,3], 1, 1, 3)]
    #[case(vec![5,4], vec![0,0,0], 1, 5, 1)]
    #[case(vec![10,15,30], vec![0,10,10,10,10], 3, 10, 2)]
    fn case(
        #[case] tasks: Vec<i32>,
        #[case] workers: Vec<i32>,
        #[case] pills: i32,
        #[case] strength: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::max_task_assign(tasks, workers, pills, strength);
        assert_eq!(actual, expected);
    }
}
