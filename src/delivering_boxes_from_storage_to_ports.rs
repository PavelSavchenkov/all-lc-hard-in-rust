//! Solution for https://leetcode.com/problems/delivering-boxes-from-storage-to-ports
//! 1687. Delivering Boxes from Storage to Ports

struct Box {
    dest: usize,
    weight: u32,
}

impl Box {
    fn new(v: &Vec<i32>) -> Self {
        Self {
            dest: v[0] as usize,
            weight: v[1] as u32,
        }
    }
}

impl Solution {
    pub fn box_delivering(
        boxes: Vec<Vec<i32>>,
        ports_count: i32,
        max_boxes: i32,
        max_weight: i32,
    ) -> i32 {
        let boxes: Vec<_> = boxes.iter().map(|b| Box::new(&b)).collect();
        let n = boxes.len();
        let ports_count = ports_count as usize;
        let max_boxes = max_boxes as usize;
        let max_weight = max_weight as u32;

        let mut need_trip = vec![0; n];
        for i in 0..n - 1 {
            if boxes[i].dest != boxes[i + 1].dest {
                need_trip[i] = 1;
            }
        }
        let mut sum_need_trip = vec![0; n + 1];
        for i in 0..n {
            sum_need_trip[i + 1] = sum_need_trip[i] + need_trip[i];
        }
        let mut next_diff_dest = vec![n; n];
        for i in (0..n - 1).rev() {
            if boxes[i].dest != boxes[i + 1].dest {
                next_diff_dest[i] = i + 1;
            } else {
                next_diff_dest[i] = next_diff_dest[i + 1];
            }
        }
        let mut dp = vec![usize::MAX; n + 1];
        dp[0] = 0;
        let mut j = 0;
        let mut cnt_boxes = 0;
        let mut sum_weight = 0;
        for i in 0..=n {
            while cnt_boxes > max_boxes || sum_weight > max_weight {
                assert!(j < i);
                cnt_boxes -= 1;
                sum_weight -= boxes[j].weight;
                j += 1;
            }

            for s in [j, next_diff_dest[j]] {
                if s >= i {
                    continue;
                }
                let mut cand = dp[s];
                assert!(cand < usize::MAX);
                cand += 2;
                cand += sum_need_trip[i - 1] - sum_need_trip[s];
                dp[i] = dp[i].min(cand);
            }

            if i < n {
                cnt_boxes += 1;
                sum_weight += boxes[i].weight;
            }
        }
        let ans = dp[n];
        assert!(ans < usize::MAX);
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
    #[case(vec![vec![1,1],vec![2,1],vec![1,1]], 2, 3, 3, 4)]
    #[case(vec![vec![1,2],vec![3,3],vec![3,1],vec![3,1],vec![2,4]], 3, 3, 6, 6)]
    #[case(vec![vec![1,4],vec![1,2],vec![2,1],vec![2,1],vec![3,2],vec![3,4]], 3, 6, 7, 6)]
    fn case(
        #[case] boxes: Vec<Vec<i32>>,
        #[case] ports_count: i32,
        #[case] max_boxes: i32,
        #[case] max_weight: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::box_delivering(boxes, ports_count, max_boxes, max_weight);
        assert_eq!(actual, expected);
    }
}
