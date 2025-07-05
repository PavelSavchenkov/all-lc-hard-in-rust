//! Solution for https://leetcode.com/problems/minimum-total-cost-to-make-arrays-unequal
//! 2499. Minimum Total Cost to Make Arrays Unequal

impl Solution {
    pub fn minimum_total_cost(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let n = a.len();
        assert!(n == b.len());
        let a: Vec<_> = a.iter().map(|&x| (x - 1) as usize).collect();
        let b: Vec<_> = b.iter().map(|&x| (x - 1) as usize).collect();

        let mut cnt_eq = vec![0; n];
        let mut mx_val = 0;
        let mut ans: u64 = 0;
        for i in 0..n {
            if a[i] == b[i] {
                ans += i as u64;
                cnt_eq[a[i]] += 1;
                if cnt_eq[mx_val] < cnt_eq[a[i]] {
                    mx_val = a[i];
                }
            }
        }
        let rest_eq = cnt_eq.iter().sum::<i32>() - cnt_eq[mx_val];
        let mut need = cnt_eq[mx_val] - rest_eq;
        if need > 0 {
            for i in 0..n {
                if a[i] != b[i] && a[i] != mx_val && b[i] != mx_val {
                    ans += i as u64;
                    need -= 1;
                    if need == 0 {
                        break;
                    }
                }
            }
            if need == 0 {
                return ans as i64;
            }
            return -1;
        }

        if (cnt_eq[mx_val] + rest_eq) % 2 == 0 {
            return ans as i64;
        }

        let mut id_inside = n;
        for i in 0..n {
            if a[i] == b[i] {
                id_inside = i;
                break;
            }
        }
        assert!(id_inside < n);

        let mut min_id_for = vec![n; n];
        let mut min_id = n;
        for i in 0..n {
            if a[i] != b[i] {
                min_id = i;
                break;
            }
        }
        if min_id == n {
            return (ans + id_inside as u64) as i64;
        }
        let mut id_outside = n;
        for i in 0..n {
            if a[i] == b[i] {
                let val = a[i];
                if val != a[min_id] && val != b[min_id] {
                    id_outside = id_outside.min(min_id);
                    continue;
                }
                if min_id_for[val] == n {
                    min_id_for[val] = n + 1;
                    for j in 0..n {
                        if a[j] != b[j] && a[j] != val && b[j] != val {
                            min_id_for[val] = j;
                            break;
                        }
                    }
                }
                let cur_min_id = min_id_for[val];
                id_outside = id_outside.min(cur_min_id);
            }
        }

        let global_min_id = id_inside.min(id_outside);
        assert!(global_min_id < n);

        ans += global_min_id as u64;

        ans as i64
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4,5], vec![1,2,3,4,5], 10)]
    #[case(vec![2,2,2,1,3], vec![1,2,2,3,3], 10)]
    #[case(vec![1,2,2], vec![1,2,2], -1)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::minimum_total_cost(nums1, nums2);
        assert_eq!(actual, expected);
    }
}
