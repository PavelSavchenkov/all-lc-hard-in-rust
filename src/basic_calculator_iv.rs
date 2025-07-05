//! Solution for https://leetcode.com/problems/basic-calculator-iv
//! 770. Basic Calculator IV

use std::collections::HashMap;

struct EvalMap {
    map: HashMap<Vec<u8>, i32>,
}

impl EvalMap {
    fn new(evalvars: Vec<String>, evalints: Vec<i32>) -> Self {
        let n = evalvars.len();
        assert!(evalints.len() == n);
        let mut map = HashMap::new();
        for i in 0..evalvars.len() {
            map.insert(to_u8(&evalvars[i]), evalints[i]);
        }
        Self { map }
    }

    fn get(&self, var: &Vec<u8>) -> Option<i32> {
        self.map.get(var).copied()
    }
}

#[derive(Clone)]
struct Monom {
    coef: i32,
    degs: HashMap<Vec<u8>, u32>,
}

impl Monom {
    fn new_from_val(val: i32) -> Self {
        Self {
            coef: val,
            degs: HashMap::new(),
        }
    }

    fn new_from_var(var: Vec<u8>) -> Self {
        let mut degs = HashMap::new();
        degs.insert(var, 1);
        Self { coef: 1, degs }
    }

    fn mul_with_val(&mut self, val: i32) {
        self.coef *= val;
        if self.coef == 0 {
            self.degs.clear();
        }
    }

    fn mul(&self, other: &Self) -> Self {
        let coef = self.coef * other.coef;
        if coef == 0 {
            return Self {
                coef,
                degs: HashMap::new(),
            };
        }
        let mut degs = self.degs.clone();
        for (var, deg) in &other.degs {
            *degs.entry(var.clone()).or_insert(0) += deg;
        }
        Self { coef, degs }
    }

    fn degree(&self) -> u32 {
        let mut res = 0;
        for (_, deg) in &self.degs {
            res += deg;
        }
        res
    }

    fn to_string_no_coef(&self) -> String {
        let mut vars = Vec::new();
        for (var, deg) in &self.degs {
            for it in 0..*deg {
                vars.push(from_u8(&var));
            }
        }
        vars.sort();
        vars.join("*")
    }
}

#[derive(Default)]
struct Poly {
    monoms: Vec<Monom>,
}

impl Poly {
    fn new() -> Self {
        Self::default()
    }

    fn new_from_monom(monom: Monom) -> Self {
        Self {
            monoms: vec![monom; 1],
        }
    }

    fn mul_with_val(&mut self, val: i32) {
        if val == 1 {
            return;
        }
        for monom in &mut self.monoms {
            monom.mul_with_val(val);
        }
        self.normalize();
    }

    fn mul_with(&mut self, other: &Poly) {
        let mut res = Vec::new();
        for monom in &self.monoms {
            for other_monom in &other.monoms {
                res.push(monom.mul(other_monom));
            }
        }
        self.monoms = res;
        self.normalize();
    }

    fn add_with(&mut self, other: &Poly) {
        self.monoms.extend_from_slice(&other.monoms[..]);
        self.normalize();
    }

    fn normalize(&mut self) {
        let mut pos_of = HashMap::<String, usize>::new();
        let mut new_monoms = Vec::<Monom>::new();
        for i in 0..self.monoms.len() {
            let key = self.monoms[i].to_string_no_coef();
            let pos = pos_of.get(&key);
            if pos.is_some() {
                let pos = *pos.unwrap();
                new_monoms[pos].coef += self.monoms[i].coef;
            } else {
                pos_of.insert(key, new_monoms.len());
                new_monoms.push(self.monoms[i].clone());
            }
        }
        self.monoms = new_monoms;
    }

    fn to_tokens(&self) -> Vec<String> {
        let mut ord: Vec<_> = (0..self.monoms.len()).collect();
        ord.sort_by_cached_key(|&i| {
            let m = &self.monoms[i];
            (-(m.degree() as i32), m.to_string_no_coef())
        });
        let mut res = Vec::new();
        for &i in &ord {
            let str = self.monoms[i].to_string_no_coef();
            let coef = self.monoms[i].coef;
            if coef == 0 {
                continue;
            }
            let coef = coef.to_string();
            if str.is_empty() {
                res.push(coef);
            } else {
                res.push(vec![coef, str].join("*"));
            }
        }
        res
    }
}

fn parse_chunk(s: &Vec<u8>, i: usize, eval_map: &EvalMap) -> (Poly, usize) {
    // eprintln!("parse_chunk {}", i);
    if s[i] == b'(' {
        let (poly, end) = parse_expression(s, i + 1, eval_map);
        assert!(s[end] == b')');
        return (poly, end + 1);
    }
    if s[i].is_ascii_lowercase() {
        let mut var = Vec::new();
        let mut j = i;
        while j < s.len() && s[j].is_ascii_lowercase() {
            var.push(s[j]);
            j += 1;
        }
        let value = eval_map.get(&var);
        let monom = if value.is_none() {
            // eprintln!("return chunk from var = {}", from_u8(&var));
            Monom::new_from_var(var)
        } else {
            // eprintln!(
            //     "return chunk from var = {} to value = {}",
            //     from_u8(&var),
            //     value.unwrap()
            // );
            Monom::new_from_val(value.unwrap())
        };
        let poly = Poly::new_from_monom(monom);
        // eprintln!("chunk poly = {:#?}", poly.to_tokens());
        return (poly, j);
    }
    assert!(s[i].is_ascii_digit());
    let mut num = 0;
    let mut j = i;
    while j < s.len() && s[j].is_ascii_digit() {
        num = num * 10 + (s[j] - b'0') as i32;
        j += 1;
    }
    let monom = Monom::new_from_val(num);
    (Poly::new_from_monom(monom), j)
}

fn parse_mul(s: &Vec<u8>, mut i: usize, eval_map: &EvalMap) -> (Poly, usize) {
    // eprintln!("parse_mul i = {}", i);
    let monom_one = Monom::new_from_val(1);
    let mut poly_pref = Poly::new_from_monom(monom_one);
    loop {
        let (poly, end) = parse_chunk(s, i, eval_map);
        poly_pref.mul_with(&poly);
        if end == s.len() || s[end] != b' ' || s[end + 1] != b'*' {
            // eprintln!("mul poly_pref = {:#?}", poly_pref.to_tokens());
            return (poly_pref, end);
        }
        i = end;
        i += 2;
        assert!(s[i] == b' ');
        i += 1;
    }
}

fn parse_expression(s: &Vec<u8>, mut i: usize, eval_map: &EvalMap) -> (Poly, usize) {
    let mut poly_pref = Poly::new();
    let mut sign = 1;
    loop {
        let (mut poly, end) = parse_mul(s, i, eval_map);
        poly.mul_with_val(sign);
        poly_pref.add_with(&poly);

        if end == s.len() || s[end] == b')' {
            return (poly_pref, end);
        }
        i = end;
        assert!(s[i] == b' ');
        i += 1;
        assert!(s[i] == b'+' || s[i] == b'-');
        if s[i] == b'-' {
            sign = -1;
        } else {
            sign = 1;
        }
        i += 1;
        assert!(s[i] == b' ');
        i += 1;
    }
}

impl Solution {
    pub fn basic_calculator_iv(
        expression: String,
        evalvars: Vec<String>,
        evalints: Vec<i32>,
    ) -> Vec<String> {
        let eval_map = EvalMap::new(evalvars, evalints);
        let expression = to_u8(&expression);
        let (poly, end) = parse_expression(&expression, 0, &eval_map);
        assert!(end == expression.len());
        poly.to_tokens()
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("e + 8 - a + 5", vec!["e".into()], vec![1], vec!["-1*a".into(),"14".into()])]
    #[case("e - 8 + temperature - pressure", vec!["e".into(), "temperature".into()], vec![1, 12], vec!["-1*pressure".into(),"5".into()])]
    #[case("(e + 8) * (e - 8)", vec![], vec![], vec!["1*e*e".into(),"-64".into()])]
    fn case(
        #[case] expression: String,
        #[case] evalvars: Vec<String>,
        #[case] evalints: Vec<i32>,
        #[case] expected: Vec<String>,
    ) {
        let actual = Solution::basic_calculator_iv(expression, evalvars, evalints);
        assert_eq!(actual, expected);
    }
}
