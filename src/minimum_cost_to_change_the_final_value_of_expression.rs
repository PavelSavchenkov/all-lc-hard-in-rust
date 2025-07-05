//! Solution for https://leetcode.com/problems/minimum-cost-to-change-the-final-value-of-expression
//! 1896. Minimum Cost to Change the Final Value of Expression

#[derive(Eq, PartialEq, Copy, Clone)]
enum Op {
    And,
    Or,
}

#[derive(Copy, Clone)]
struct OpNode {
    left: usize,
    right: usize,
    op: Op,
}

#[derive(Copy, Clone)]
struct ValNode {
    val: usize,
}

#[derive(Copy, Clone)]
enum Node {
    OpNode(OpNode),
    ValNode(ValNode),
}

fn remin(a: &mut usize, b: usize) {
    if *a > b {
        *a = b;
    }
}

#[derive(Default)]
struct Parser {
    nodes: Vec<Node>,
    root: usize,
    dp: Vec<Vec<usize>>,
}

impl Parser {
    fn new(s: &Vec<u8>) -> Self {
        let mut parser = Self::default();
        let (i, root) = parser.parse(s, 0);
        assert!(i == s.len());
        parser.root = root;
        parser
    }

    fn get_ans(&mut self) -> usize {
        self.dp = vec![vec![usize::MAX; 2]; self.nodes.len()];
        self.calc_dp(self.root);
        *self.dp[self.root].iter().max().unwrap()
    }

    fn calc_dp(&mut self, v: usize) {
        match self.nodes[v] {
            Node::OpNode(op_node) => {
                self.calc_dp(op_node.left);
                self.calc_dp(op_node.right);
                for val_left in 0..2 {
                    for val_right in 0..2 {
                        let val_cost =
                            self.dp[op_node.left][val_left] + self.dp[op_node.right][val_right];
                        // &
                        let and_res = val_left & val_right;
                        let and_cost = if op_node.op == Op::And { 0 } else { 1 };
                        remin(&mut self.dp[v][and_res], val_cost + and_cost);
                        // |
                        let or_res = val_left | val_right;
                        let or_cost = if op_node.op == Op::Or { 0 } else { 1 };
                        remin(&mut self.dp[v][or_res], val_cost + or_cost);
                    }
                }
            }
            Node::ValNode(val_node) => {
                self.dp[v][val_node.val] = 0;
                self.dp[v][val_node.val ^ 1] = 1;
            }
        }
    }

    fn push_node(&mut self, node: Node) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    fn parse_atom(&mut self, s: &Vec<u8>, i: usize) -> (usize, usize) {
        assert!(i < s.len());
        if s[i] == b'(' {
            let (i, v) = self.parse(s, i + 1);
            assert!(s[i] == b')');
            return (i + 1, v);
        }
        match s[i] {
            b'0' => (i + 1, self.push_node(Node::ValNode(ValNode { val: 0 }))),
            b'1' => (i + 1, self.push_node(Node::ValNode(ValNode { val: 1 }))),
            _ => panic!("This char should be either 0 or 1"),
        }
    }

    fn parse_op(s: &Vec<u8>, i: usize) -> Op {
        match s[i] {
            b'|' => Op::Or,
            b'&' => Op::And,
            _ => panic!("Op should be either | or &"),
        }
    }

    fn parse(&mut self, s: &Vec<u8>, mut i: usize) -> (usize, usize) {
        assert!(i < s.len());

        let (end, mut root) = self.parse_atom(s, i);
        i = end;
        loop {
            if i >= s.len() || s[i] == b')' {
                break;
            }
            let op = Self::parse_op(s, i);
            i += 1;
            let (end, atom) = self.parse_atom(s, i);
            i = end;

            let op_node = OpNode {
                left: root,
                right: atom,
                op: op,
            };
            root = self.push_node(Node::OpNode(op_node));
        }
        (i, root)
    }
}

impl Solution {
    pub fn min_operations_to_flip(expression: String) -> i32 {
        let s = to_u8(&expression);
        let mut parser = Parser::new(&s);
        let ans = parser.get_ans();
        ans as i32
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
    #[case("1&(0|1)", 1)]
    #[case("(0&0)&(0&0&0)", 3)]
    #[case("(0|(1|0&1))", 1)]
    fn case(#[case] expression: String, #[case] expected: i32) {
        let actual = Solution::min_operations_to_flip(expression);
        assert_eq!(actual, expected);
    }
}
