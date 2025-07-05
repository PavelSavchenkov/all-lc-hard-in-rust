//! Solution for https://leetcode.com/problems/maximum-number-of-groups-with-increasing-length
//! 2790. Maximum Number of Groups With Increasing Length

use std::collections::BTreeMap;

struct MySet {
    map: BTreeMap<i32, usize>, // (val, cnt of such val)
    val_offset: i32,
    sum_cnt: usize,
}

impl MySet {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            val_offset: 0,
            sum_cnt: 0,
        }
    }

    fn subtract_all(&mut self, delta: i32) {
        self.val_offset -= delta;
    }

    fn insert(&mut self, val: i32, cnt: usize) {
        *self.map.entry(val - self.val_offset).or_insert(0) += cnt;
        self.sum_cnt += cnt;
    }

    fn remove_max(&mut self) -> Option<(i32, usize)> {
        let max = self.map.pop_last();
        if max.is_none() {
            return None;
        }
        let (val, cnt) = max.unwrap();
        self.sum_cnt -= cnt;
        Some((val + self.val_offset, cnt))
    }

    fn remove_min(&mut self) -> Option<(i32, usize)> {
        let min = self.map.pop_first();
        if min.is_none() {
            return None;
        }
        let (val, cnt) = min.unwrap();
        self.sum_cnt -= cnt;
        Some((val + self.val_offset, cnt))
    }

    fn min_value(&self) -> Option<i32> {
        if self.map.is_empty() {
            return None;
        }
        Some(*self.map.first_key_value().unwrap().0 + self.val_offset)
    }

    fn max_value(&self) -> Option<i32> {
        if self.map.is_empty() {
            return None;
        }
        Some(*self.map.last_key_value().unwrap().0 + self.val_offset)
    }
}

impl Solution {
    pub fn max_increasing_groups(a: Vec<i32>) -> i32 {
        let can = |a: Vec<i32>, k: usize| -> bool {
            // subtract one from 1 maximum element
            // subtract one from 2 maximum elements
            // etc
            let n = a.len();
            assert!(k <= n);
            let mut active = MySet::new();
            let mut passive = MySet::new();
            for i in 0..n {
                passive.insert(a[i], 1);
            }
            for i in 1..=k {
                assert!(active.sum_cnt + passive.sum_cnt == n);
                while active.sum_cnt < i {
                    assert!(passive.sum_cnt > 0);
                    let (val, cnt) = passive.remove_max().unwrap();
                    let take = (i - active.sum_cnt).min(cnt);
                    active.insert(val, take);
                    if cnt > take {
                        passive.insert(val, cnt - take);
                    }
                }

                while passive.sum_cnt > 0
                    && passive.max_value().unwrap() > active.min_value().unwrap()
                {
                    let (val_add, cnt_add) = passive.remove_max().unwrap();
                    let (val_del, cnt_del) = active.remove_min().unwrap();
                    let cnt_move = cnt_add.min(cnt_del);
                    assert!(cnt_move > 0);
                    active.insert(val_add, cnt_move);
                    passive.insert(val_del, cnt_move);
                    if cnt_move < cnt_add {
                        passive.insert(val_add, cnt_add - cnt_move);
                    }
                    if cnt_move < cnt_del {
                        active.insert(val_del, cnt_del - cnt_move);
                    }
                }

                assert!(active.sum_cnt == i);
                active.subtract_all(1);
                if active.min_value().unwrap() < 0 {
                    return false;
                }
            }
            true
        };

        let n = a.len();
        let mut a: Vec<_> = a.iter().map(|&x| x.min(n as i32)).collect();
        a.sort();

        let mut L = 0;
        let mut R = n + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if can(a.clone(), M) {
                L = M;
            } else {
                R = M;
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
    #[case(vec![1,2,5], 3)]
    #[case(vec![2,1,2], 2)]
    #[case(vec![1,1], 1)]
    #[case(vec![2,2,2], 3)]
    fn case(#[case] usage_limits: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_increasing_groups(usage_limits);
        assert_eq!(actual, expected);
    }
}
