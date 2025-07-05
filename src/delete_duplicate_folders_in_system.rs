//! Solution for https://leetcode.com/problems/delete-duplicate-folders-in-system
//! 1948. Delete Duplicate Folders in System

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

struct Node {
    name: u64,
    children: HashMap<u64, usize>, // child name to index in "nodes"
}

impl Node {
    fn new(name: u64) -> Node {
        Node {
            name,
            children: HashMap::new(),
        }
    }
}

fn dfs_ans(
    v: usize,
    nodes: &Vec<Node>,
    killed: &[bool],
    path: &mut Vec<String>,
    ans: &mut Vec<Vec<String>>,
    hash_to_name: &HashMap<u64, String>,
) {
    if v != 0 {
        let str_name = hash_to_name.get(&nodes[v].name).unwrap().to_string();
        path.push(str_name);
    }
    assert!(!killed[v]);

    for (_, &child) in nodes[v].children.iter() {
        if killed[child] {
            continue;
        }
        dfs_ans(child, nodes, killed, path, ans, hash_to_name);
    }

    if v != 0 {
        ans.push(path.clone());
        path.pop();
    }
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut hash_to_name = HashMap::<u64, String>::new();
        let mut ps = vec![Vec::<u64>::new(); paths.len()];
        for path in paths {
            let mut p = vec![];
            for s in path {
                let h = get_hash(&s);
                hash_to_name.insert(h, s.clone());
                p.push(h);
            }
            ps.push(p);
        }

        let mut nodes = Vec::<Node>::new();
        nodes.push(Node::new(0));

        // build directory tree
        for p in ps {
            let mut v = 0;
            for name in p {
                if !nodes[v].children.contains_key(&name) {
                    let new_idx = nodes.len();
                    nodes.push(Node::new(name));
                    nodes[v].children.insert(name, new_idx);
                }
                v = *nodes[v].children.get(&name).unwrap();
            }
        }

        let mut nodes_with_such_hash = HashMap::<u64, Vec<usize>>::new();
        let mut node_h = vec![0 as u64; nodes.len()];
        for i in (0..nodes.len()).rev() {
            let mut children_h = vec![];
            for (_, &v) in nodes[i].children.iter() {
                children_h.push(node_h[v]);
            }
            children_h.sort();
            let children_h = get_hash(&children_h);
            if !nodes[i].children.is_empty() {
                nodes_with_such_hash
                    .entry(children_h)
                    .or_insert(vec![])
                    .push(i);
            }
            let final_hash = get_hash(&[nodes[i].name, children_h]);
            node_h[i] = final_hash;
        }

        let mut killed = vec![false; nodes.len()];
        for (_, v) in nodes_with_such_hash.iter() {
            if v.len() == 1 {
                continue;
            }
            assert!(!v.is_empty());
            for &idx in v {
                killed[idx] = true;
            }
        }

        let mut ans = Vec::<Vec<String>>::new();
        let mut path = Vec::<String>::new();
        dfs_ans(0, &nodes, &killed, &mut path, &mut ans, &hash_to_name);
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
    #[case(vec![vec!["a".into()],vec!["c".into()],vec!["d".into()],vec!["a".into(),"b".into()],vec!["c".into(),"b".into()],vec!["d".into(),"a".into()]], vec![vec!["d".into()],vec!["d".into(),"a".into()]])]
    #[case(vec![vec!["a".into()],vec!["c".into()],vec!["a".into(),"b".into()],vec!["c".into(),"b".into()],vec!["a".into(),"b".into(),"x".into()],vec!["a".into(),"b".into(),"x".into(),"y".into()],vec!["w".into()],vec!["w".into(),"y".into()]], vec![vec!["c".into()],vec!["c".into(),"b".into()],vec!["a".into()],vec!["a".into(),"b".into()]])]
    #[case(vec![vec!["a".into(),"b".into()],vec!["c".into(),"d".into()],vec!["c".into()],vec!["a".into()]], vec![vec!["c".into()],vec!["c".into(),"d".into()],vec!["a".into()],vec!["a".into(),"b".into()]])]
    fn case(#[case] paths: Vec<Vec<String>>, #[case] expected: Vec<Vec<String>>) {
        let actual = Solution::delete_duplicate_folder(paths);
        assert_eq!(actual, expected);
    }
}
