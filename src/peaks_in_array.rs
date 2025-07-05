//! Solution for https://leetcode.com/problems/peaks-in-array
//! 3187. Peaks in Array

#[derive(Clone)]
struct FenwTreeSum {
    t: Vec<i64>,
}

impl FenwTreeSum {
    fn new(n: usize) -> Self {
        Self { t: vec![0; n] }
    }

    fn add_point(&mut self, pos: usize, val: i64) {
        let mut i = pos;
        while i < self.t.len() {
            self.t[i] += val;
            i |= i + 1;
        }
    }

    // [0; r)
    fn get_sum_pref(&self, r: usize) -> i64 {
        if r == 0 {
            return 0;
        }
        let mut sum = 0;
        let mut i = r - 1;
        loop {
            sum += self.t[i];
            i &= i + 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        sum
    }

    fn get_sum(&self, l: usize, r: usize) -> i64 {
        if l >= r {
            return 0;
        }
        self.get_sum_pref(r) - self.get_sum_pref(l)
    }
}

impl Solution {
    pub fn count_of_peaks(mut a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = a.len();

        let update = |tree: &mut FenwTreeSum, a: &Vec<i32>, pos: usize, is_add: bool| {
            if pos == 0 || pos >= n - 1 {
                return;
            }
            if a[pos - 1] < a[pos] && a[pos] > a[pos + 1] {
                if is_add {
                    tree.add_point(pos, 1);
                } else {
                    tree.add_point(pos, -1);
                }
            }
        };

        let mut tree = FenwTreeSum::new(n);

        for i in 1..n - 1 {
            update(&mut tree, &a, i, true);
        }

        let mut ans = Vec::new();
        for q in &queries {
            match q[0] {
                1 => {
                    let l = q[1] as usize;
                    let r = q[2] as usize;
                    let mut cnt = 0;
                    if r - l + 1 >= 3 {
                        cnt = tree.get_sum(l + 1, r) as i32;
                    }
                    ans.push(cnt);
                }
                2 => {
                    let i = q[1];
                    let val = q[2];
                    for j in i - 1..=i + 1 {
                        if j >= 0 {
                            update(&mut tree, &a, j as usize, false);
                        }
                    }
                    a[i as usize] = val;
                    for j in i - 1..=i + 1 {
                        if j >= 0 {
                            update(&mut tree, &a, j as usize, true);
                        }
                    }
                }
                _ => panic!(),
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
    #[case(vec![3,1,4,2,5], vec![vec![2,3,4],vec![1,0,4]], vec![0])]
    #[case(vec![4,1,4,2,1,5], vec![vec![2,2,4],vec![1,0,2],vec![1,0,4]], vec![0,1])]
    fn case(#[case] nums: Vec<i32>, #[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::count_of_peaks(nums, queries);
        assert_eq!(actual, expected);
    }
}
