//! Solution for https://leetcode.com/problems/reconstruct-itinerary
//! 332. Reconstruct Itinerary

impl Solution {
    pub fn find_itinerary(mut tickets: Vec<Vec<String>>) -> Vec<String> {
        // tickets = vec![
        //     vec!["JFK".to_string(), "ATL".to_string()],
        //     vec!["ORD".to_string(), "PHL".to_string()],
        //     vec!["JFK".to_string(), "ORD".to_string()],
        //     vec!["PHX".to_string(), "LAX".to_string()],
        //     vec!["LAX".to_string(), "JFK".to_string()],
        //     vec!["PHL".to_string(), "ATL".to_string()],
        //     vec!["ATL".to_string(), "PHX".to_string()],
        // ];

        let mut airports = Vec::new();
        for e in &tickets {
            airports.push(e[0].clone());
            airports.push(e[1].clone());
        }
        airports.sort();
        airports.dedup();

        // eprintln!("airports = {:#?}", airports);

        let n = airports.len();
        let mut g = vec![Vec::new(); n];
        let mut in_minus_out = vec![0; n];
        for e in &tickets {
            let from = airports.binary_search(&e[0]).unwrap();
            let to = airports.binary_search(&e[1]).unwrap();
            g[from].push(to);
            in_minus_out[from] -= 1;
            in_minus_out[to] += 1;
        }

        let source = airports.binary_search(&"JFK".to_string()).unwrap();
        let mut sink = None;
        for v in 0..n {
            if in_minus_out[v] == 1 {
                assert!(sink.is_none());
                sink = Some(v);
            }
        }
        let is_eulerian_cycle = sink.is_none();

        // if sink.is_some() {
        //     g[sink.unwrap()].push(source);
        // }

        for v in 0..n {
            g[v].sort();
            g[v].reverse();
        }

        let mut st = Vec::<usize>::new();
        if is_eulerian_cycle {
            st.push(source);
        } else {
            st.push(sink.unwrap());
            st.push(source);
        }
        let mut ans = Vec::new();
        while !st.is_empty() {
            let v = *st.last().unwrap();
            // eprintln!("st = {:#?}", st);
            if g[v].is_empty() {
                ans.push(v);
                // eprintln!("{} --> ans", v);
                st.pop();
            } else {
                let to = g[v].last().unwrap().clone();
                g[v].pop();
                st.push(to);
            }
        }
        // eprintln!("ans = {:#?}", ans);
        ans.reverse();
        // eprintln!("ans = {:#?}", ans);

        if !is_eulerian_cycle {
            ans.pop();
            let sink = sink.unwrap();
            let mut found_extra_edge = false;
            for i in 0..ans.len() {
                if ans[i] == sink && ans[(i + 1) % ans.len()] == source {
                    let mut new_ans = Vec::new();
                    let mut j = (i + 1) % ans.len();
                    for it in 0..ans.len() {
                        new_ans.push(ans[j]);
                        j += 1;
                        if j == ans.len() {
                            j = 0;
                        }
                    }
                    ans = new_ans;
                    found_extra_edge = true;
                    break;
                }
            }
            assert!(found_extra_edge);
        }

        ans.iter().map(|&v| airports[v].clone()).collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec!["MUC".into(),"LHR".into()],vec!["JFK".into(),"MUC".into()],vec!["SFO".into(),"SJC".into()],vec!["LHR".into(),"SFO".into()]], vec!["JFK".into(),"MUC".into(),"LHR".into(),"SFO".into(),"SJC".into()])]
    #[case(vec![vec!["JFK".into(),"SFO".into()],vec!["JFK".into(),"ATL".into()],vec!["SFO".into(),"ATL".into()],vec!["ATL".into(),"JFK".into()],vec!["ATL".into(),"SFO".into()]], vec!["JFK".into(),"ATL".into(),"JFK".into(),"SFO".into(),"ATL".into(),"SFO".into()])]
    fn case(#[case] tickets: Vec<Vec<String>>, #[case] expected: Vec<String>) {
        let actual = Solution::find_itinerary(tickets);
        assert_eq!(actual, expected);
    }
}
