//! Solution for https://leetcode.com/problems/valid-arrangement-of-pairs
//! 2097. Valid Arrangement of Pairs

// Eulerian path in directed graph

// #[derive(Copy, Clone, Debug)]
struct Edge {
    from: usize,
    to: usize,
}

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut values = Vec::new();
        for p in &pairs {
            values.push(p[0]);
            values.push(p[1]);
        }
        values.sort();
        values.dedup();
        let n = values.len();
        let mut edges = Vec::new();
        for (i, p) in pairs.iter().enumerate() {
            let from = p[0];
            let to = p[1];
            let from = values.binary_search(&from).unwrap();
            let to = values.binary_search(&to).unwrap();
            edges.push(Edge { from, to });
        }

        let mut out_minus_in = vec![0 as i32; n];
        for e in &edges {
            out_minus_in[e.from] += 1;
            out_minus_in[e.to] -= 1;
        }

        let mut source = None;
        let mut sink = None;
        for v in 0..n {
            assert!(out_minus_in[v].abs() <= 1);
            if out_minus_in[v] == 1 {
                assert!(source.is_none());
                source = Some(v);
            } else if out_minus_in[v] == -1 {
                assert!(sink.is_none());
                sink = Some(v);
            }
        }
        if !source.is_none() {
            assert!(!sink.is_none());
            let id = edges.len();
            edges.push(Edge {
                from: sink.unwrap(),
                to: source.unwrap(),
            });
        } else {
            assert!(sink.is_none());
        }

        let mut g = vec![Vec::<usize>::new(); n];
        for (i, e) in edges.iter().enumerate() {
            g[e.from].push(i);
        }

        let mut nodes_order = Vec::new();
        let mut st = Vec::new();
        let mut ptr = vec![0; n]; // index of the first unused edge
        st.push(0);
        while !st.is_empty() {
            let v = *st.last().unwrap();
            if ptr[v] == g[v].len() {
                nodes_order.push(v);
                st.pop();
            } else {
                let id = g[v][ptr[v]];
                let to = edges[id].to;
                assert!(to != v);
                ptr[v] += 1;
                st.push(to);
            }
        }
        nodes_order.reverse();

        if !source.is_none() {
            let mut new_order = Vec::new();
            for i in 0..nodes_order.len() - 1 {
                if nodes_order[i] == sink.unwrap() && nodes_order[i + 1] == source.unwrap() {
                    nodes_order.pop();
                    let mut j = (i + 1) % nodes_order.len();
                    for it in 0..nodes_order.len() {
                        new_order.push(nodes_order[j]);
                        j += 1;
                        if j == nodes_order.len() {
                            j = 0;
                        }
                    }
                    break;
                }
            }
            assert!(!new_order.is_empty());
            nodes_order = new_order;
        }

        let mut ans = Vec::new();
        for i in 0..nodes_order.len() - 1 {
            let a = nodes_order[i];
            let b = nodes_order[i + 1];
            let a = values[a];
            let b = values[b];
            ans.push(vec![a, b]);
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
    #[case(vec![vec![5,1],vec![4,5],vec![11,9],vec![9,4]], vec![vec![11,9],vec![9,4],vec![4,5],vec![5,1]])]
    #[case(vec![vec![1,3],vec![3,2],vec![2,1]], vec![vec![1,3],vec![3,2],vec![2,1]])]
    #[case(vec![vec![1,2],vec![1,3],vec![2,1]], vec![vec![1,2],vec![2,1],vec![1,3]])]
    fn case(#[case] pairs: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::valid_arrangement(pairs);
        assert_eq!(actual, expected);
    }
}
