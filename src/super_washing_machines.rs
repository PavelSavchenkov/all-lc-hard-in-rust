//! Solution for https://leetcode.com/problems/super-washing-machines
//! 517. Super Washing Machines

impl Solution {
    pub fn find_min_moves(a: Vec<i32>) -> i32 {
        let n = a.len();
        let sum: i32 = a.iter().sum();
        if sum % n as i32 != 0 {
            return -1;
        }
        let each = sum / n as i32;

        {
            let mut left = 0;
            let mut ans = 0;
            for i in 0..n {
                left += a[i];
                let need_left = (i + 1) as i32 * each;
                ans = ans.max((left - need_left).abs());
                ans = ans.max(a[i] - each);
            }
            return ans;
        }

        let mut l = 0;
        let mut l_cnt = 0;
        let mut vals = vec![Vec::new(); n];
        let mut push_val = |from: usize, to: usize, cnt: i32| {
            if from == to || cnt == 0 {
                return;
            }
            let val = (to as i32 - from as i32).abs() + (cnt - 1);
            vals[from].push(val);
        };
        for i in 0..n {
            let mut taken = 0;
            let mut r = l;
            let mut r_cnt = l_cnt;
            while taken < each {
                let need = each - taken;
                if need < a[r] - r_cnt {
                    taken += need;
                    r_cnt += need;
                } else {
                    taken += a[r] - r_cnt;
                    r += 1;
                    r_cnt = 0;
                }
            }
            eprintln!(
                "i={}, l={}, l_cnt={}, r={}, r_cnt={}",
                i, l, l_cnt, r, r_cnt
            );
            for j in l + 1..r {
                push_val(j, i, a[j]);
            }
            if l != r {
                push_val(l, i, a[l] - l_cnt);
                push_val(r, i, r_cnt);
            } else {
                push_val(l, i, r_cnt - l_cnt);
            }
            l = r;
            l_cnt = r_cnt;
        }

        let mut ans = 0;
        for i in 0..n {
            let s = vals[i].iter().sum();
            ans = ans.max(s);
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
    #[case(vec![1,0,5], 3)]
    #[case(vec![0,3,0], 2)]
    #[case(vec![0,2,0], -1)]
    fn case(#[case] machines: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::find_min_moves(machines);
        assert_eq!(actual, expected);
    }
}
