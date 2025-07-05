//! Solution for https://leetcode.com/problems/distribute-elements-into-two-arrays-ii
//! 3072. Distribute Elements Into Two Arrays II

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

#[derive(Clone)]
struct Arr {
    tree: FenwTreeSum,
    arr: Vec<usize>,
}

impl Arr {
    fn new(n: usize) -> Self {
        Self {
            tree: FenwTreeSum::new(n),
            arr: Vec::new(),
        }
    }

    fn push(&mut self, x: usize) {
        self.tree.add_point(x, 1);
        self.arr.push(x);
    }

    fn greaterCount(&self, x: usize) -> usize {
        self.tree.get_sum(x + 1, self.tree.t.len()) as usize
    }

    fn len(&self) -> usize {
        self.arr.len()
    }
}

impl Solution {
    pub fn result_array(a: Vec<i32>) -> Vec<i32> {
        let mut vals = a.clone();
        vals.sort();
        vals.dedup();

        let a: Vec<_> = a.iter().map(|&x| vals.binary_search(&x).unwrap()).collect();

        let mut arr = vec![Arr::new(vals.len()); 2];
        arr[0].push(a[0]);
        arr[1].push(a[1]);
        for i in 2..a.len() {
            let count0 = arr[0].greaterCount(a[i]);
            let count1 = arr[1].greaterCount(a[i]);
            if count0 > count1 {
                arr[0].push(a[i]);
            } else if count0 < count1 {
                arr[1].push(a[i]);
            } else if arr[0].len() < arr[1].len() {
                arr[0].push(a[i]);
            } else if arr[1].len() < arr[0].len() {
                arr[1].push(a[i]);
            } else {
                arr[0].push(a[i]);
            }
        }

        let mut ans = arr[0].arr.clone();
        ans.extend(arr[1].arr.iter());

        let ans: Vec<_> = ans.iter().map(|&x| vals[x]).collect();
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
    #[case(vec![2,1,3,3], vec![2,3,1,3])]
    #[case(vec![5,14,3,1,2], vec![5,3,1,2,14])]
    #[case(vec![3,3,3,3], vec![3,3,3,3])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::result_array(nums);
        assert_eq!(actual, expected);
    }
}
