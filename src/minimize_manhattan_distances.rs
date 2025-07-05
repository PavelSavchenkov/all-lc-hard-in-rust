//! Solution for https://leetcode.com/problems/minimize-manhattan-distances
//! 3102. Minimize Manhattan Distances

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn get_min_max(ps: &Vec<Point>) -> (i32, i32, i32, i32) {
    let min_x = ps.iter().fold(i32::MAX, |acc, e| acc.min(e.x));
    let max_x = ps.iter().fold(i32::MIN, |acc, e| acc.max(e.x));
    let min_y = ps.iter().fold(i32::MAX, |acc, e| acc.min(e.y));
    let max_y = ps.iter().fold(i32::MIN, |acc, e| acc.max(e.y));
    (min_x, max_x, min_y, max_y)
}

fn get_max(ps: Vec<Point>) -> i32 {
    let (min_x, max_x, min_y, max_y) = get_min_max(&ps);
    (max_y - min_y).max(max_x - min_x)
}

impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();

        let mut ps = Vec::with_capacity(n);
        for p in &points {
            let x = p[0];
            let y = p[1];
            ps.push(Point { x: x + y, y: x - y });
        }

        let (min_x, max_x, min_y, max_y) = get_min_max(&ps);

        if max_y - min_y > max_x - min_x {
            ps = ps.iter().map(|p| Point { x: p.y, y: p.x }).collect();
        }

        ps.sort_by_key(|p| p.x);

        let p = ps.remove(0);
        let mut ans = get_max(ps.clone());
        ps.insert(0, p);

        let p = ps.remove(n - 1);
        ans = ans.min(get_max(ps.clone()));
        ps.insert(n - 1, p);

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
    #[case(vec![vec![3,10],vec![5,15],vec![10,2],vec![4,4]], 12)]
    #[case(vec![vec![1,1],vec![1,1],vec![1,1]], 0)]
    fn case(#[case] points: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::minimum_distance(points);
        assert_eq!(actual, expected);
    }
}
