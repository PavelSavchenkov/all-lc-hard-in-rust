//! Solution for https://leetcode.com/problems/smallest-sufficient-team
//! 1125. Smallest Sufficient Team

use std::collections::HashMap;

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let mut skill_id = HashMap::new();
        for i in 0..req_skills.len() {
            skill_id.insert(req_skills[i].clone(), i);
        }

        let mut masks = Vec::new();
        for person in &people {
            let mut mask = 0;
            for skill in person {
                let id = skill_id.get(skill).unwrap();
                mask |= 1 << id;
            }
            masks.push(mask);
        }

        let n = people.len();
        let k = skill_id.len();
        let all_people: u64 = (1 << n) - 1;
        let mut dp = vec![all_people; 1 << k];
        dp[0] = 0;
        for i in 0..n {
            let mask = masks[i];
            for prev_mask in 0..1 << k {
                let new_mask = mask | prev_mask;
                if dp[new_mask].count_ones() > (dp[prev_mask] | 1 << i).count_ones() {
                    dp[new_mask] = dp[prev_mask] | 1 << i;
                }
            }
        }

        let mut ans = Vec::new();
        for i in 0..n {
            if ((dp[(1 << k) - 1] >> i) & 1) == 1 {
                ans.push(i as i32);
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
    #[case(vec!["java".into(),"nodejs".into(),"reactjs".into()], vec![vec!["java".into()],vec!["nodejs".into()],vec!["nodejs".into(),"reactjs".into()]], vec![0,2])]
    #[case(vec!["algorithms".into(),"math".into(),"java".into(),"reactjs".into(),"csharp".into(),"aws".into()], vec![vec!["algorithms".into(),"math".into(),"java".into()],vec!["algorithms".into(),"math".into(),"reactjs".into()],vec!["java".into(),"csharp".into(),"aws".into()],vec!["reactjs".into(),"csharp".into()],vec!["csharp".into(),"math".into()],vec!["aws".into(),"java".into()]], vec![1,2])]
    fn case(
        #[case] req_skills: Vec<String>,
        #[case] people: Vec<Vec<String>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::smallest_sufficient_team(req_skills, people);
        assert_eq!(actual, expected);
    }
}
