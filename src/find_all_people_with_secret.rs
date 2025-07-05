//! Solution for https://leetcode.com/problems/find-all-people-with-secret
//! 2092. Find All People With Secret

struct Meeting {
    x: usize,
    y: usize,
    time: usize,
}

impl Meeting {
    fn new(v: &Vec<i32>) -> Self {
        Self {
            x: v[0] as usize,
            y: v[1] as usize,
            time: v[2] as usize,
        }
    }
}

struct DSU {
    par: Vec<Option<usize>>,
    size: Vec<usize>,
    history: Vec<(usize, usize)>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            par: vec![None; n],
            size: vec![1; n],
            history: vec![],
        }
    }

    fn get(&self, x: usize) -> usize {
        if self.par[x].is_none() {
            x
        } else {
            self.get(self.par[x].unwrap())
        }
    }

    fn merge(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.get(x);
        y = self.get(y);

        if x == y {
            return false;
        }

        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.par[y] = Some(x);
        self.size[x] += self.size[y];
        self.history.push((x, y));
        true
    }

    fn roll_back(&mut self) {
        let (x, y) = self.history.pop().unwrap();
        self.par[y] = None;
        self.size[x] -= self.size[y];
    }
}

impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let n = n as usize;
        let first_person = first_person as usize;

        let mut has_secret = vec![false; n];
        has_secret[0] = true;

        let mut meetings: Vec<_> = meetings.iter().map(|v| Meeting::new(v)).collect();
        meetings.push(Meeting {
            x: 0,
            y: first_person,
            time: 0,
        });
        meetings.sort_by_key(|m| m.time);

        let mut dsu = DSU::new(n);
        let mut i = 0;
        while i < meetings.len() {
            let mut j = i;
            while j < meetings.len() && meetings[j].time == meetings[i].time {
                j += 1;
            }

            let mut cnt_roll_back = 0;
            for k in i..j {
                let m = &meetings[k];
                if dsu.merge(m.x, m.y) {
                    cnt_roll_back += 1;
                }
            }

            for k in i..j {
                let m = &meetings[k];
                for z in [m.x, m.y] {
                    if has_secret[z] {
                        has_secret[dsu.get(z)] = true;
                    }
                }
            }

            for k in i..j {
                let m = &meetings[k];
                for z in [m.x, m.y] {
                    if has_secret[dsu.get(z)] {
                        has_secret[z] = true;
                    }
                }
            }

            for it in 0..cnt_roll_back {
                dsu.roll_back();
            }

            i = j;
        }

        (0..n)
            .filter(|&i| has_secret[i])
            .map(|i| i as i32)
            .collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(6, vec![vec![1,2,5],vec![2,3,8],vec![1,5,10]], 1, vec![0,1,2,3,5])]
    #[case(4, vec![vec![3,1,3],vec![1,2,2],vec![0,3,3]], 3, vec![0,1,3])]
    #[case(5, vec![vec![3,4,2],vec![1,2,1],vec![2,3,1]], 1, vec![0,1,2,3,4])]
    fn case(
        #[case] n: i32,
        #[case] meetings: Vec<Vec<i32>>,
        #[case] first_person: i32,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::find_all_people(n, meetings, first_person);
        assert_eq!(actual, expected);
    }
}
