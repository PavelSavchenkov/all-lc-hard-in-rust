//! Solution for https://leetcode.com/problems/process-restricted-friend-requests
//! 2076. Process Restricted Friend Requests

struct DSU {
    p: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self { p: vec![-1; n] }
    }

    fn get(&mut self, x: usize) -> usize {
        if self.p[x] < 0 {
            x
        } else {
            self.p[x] = self.get(self.p[x] as usize) as i32;
            self.p[x] as usize
        }
    }

    fn merge(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.get(x);
        y = self.get(y);
        if x == y {
            return false;
        }
        if -self.p[x] < -self.p[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.p[x] += self.p[y];
        self.p[y] = x as i32;
        true
    }
}

impl Solution {
    pub fn friend_requests(
        n: i32,
        restrictions: Vec<Vec<i32>>,
        requests: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = n as usize;

        let mut dsu = DSU::new(n);
        let mut ans = Vec::new();
        for request in &requests {
            let mut x = request[0] as usize;
            let mut y = request[1] as usize;
            x = dsu.get(x);
            y = dsu.get(y);

            let mut can_be_friends = true;
            for r in &restrictions {
                let mut a = r[0] as usize;
                let mut b = r[1] as usize;
                for it in 0..2 {
                    if dsu.get(a) == x && dsu.get(b) == y {
                        can_be_friends = false;
                        break;
                    }
                    std::mem::swap(&mut a, &mut b);
                }
            }
            ans.push(can_be_friends);
            if can_be_friends {
                dsu.merge(x, y);
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
    #[case(3, vec![vec![0,1]], vec![vec![0,2],vec![2,1]], vec![true,false])]
    #[case(3, vec![vec![0,1]], vec![vec![1,2],vec![0,2]], vec![true,false])]
    #[case(5, vec![vec![0,1],vec![1,2],vec![2,3]], vec![vec![0,4],vec![1,2],vec![3,1],vec![3,4]], vec![true,false,true,false])]
    fn case(
        #[case] n: i32,
        #[case] restrictions: Vec<Vec<i32>>,
        #[case] requests: Vec<Vec<i32>>,
        #[case] expected: Vec<bool>,
    ) {
        let actual = Solution::friend_requests(n, restrictions, requests);
        assert_eq!(actual, expected);
    }
}
