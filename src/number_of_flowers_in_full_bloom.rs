//! Solution for https://leetcode.com/problems/number-of-flowers-in-full-bloom
//! 2251. Number of Flowers in Full Bloom

enum EventType {
    Start,
    End,
    Person(usize),
}

struct Event {
    x: u32,
    t: EventType,
}

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut es = Vec::new();
        for f in &flowers {
            let l = f[0] as u32;
            let r = (f[1] + 1) as u32;
            es.push(Event {
                x: l,
                t: EventType::Start,
            });
            es.push(Event {
                x: r,
                t: EventType::End,
            });
        }

        for i in 0..people.len() {
            let x = people[i] as u32;
            es.push(Event {
                x,
                t: EventType::Person(i),
            });
        }

        es.sort_by_key(|e| e.x);

        let mut ans = vec![0; people.len()];
        let mut cnt: i32 = 0;
        for e in &es {
            match e.t {
                EventType::Start => {
                    cnt += 1;
                }
                EventType::End => {
                    cnt -= 1;
                }
                EventType::Person(id) => {
                    ans[id] = cnt;
                }
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
    #[case(vec![vec![1,6],vec![3,7],vec![9,12],vec![4,13]], vec![2,3,7,11], vec![1,2,2,2])]
    #[case(vec![vec![1,10],vec![3,3]], vec![3,3,2], vec![2,2,1])]
    fn case(#[case] flowers: Vec<Vec<i32>>, #[case] people: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::full_bloom_flowers(flowers, people);
        assert_eq!(actual, expected);
    }
}
