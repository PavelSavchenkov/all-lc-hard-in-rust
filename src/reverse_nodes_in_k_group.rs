//! Solution for https://leetcode.com/problems/reverse-nodes-in-k-group
//! 25. Reverse Nodes in k-Group

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;
        if k == 1 {
            return head;
        }

        let mut dummy = Some(Box::new(ListNode{ val: -1, next: head }));
        let mut prev = &mut dummy;
        loop {
            // check if there are k nodes ahead
            {
                let mut less_k = false;
                let mut ptr: &Option<Box<ListNode>> = &prev;
                for it in 0..k + 1 {
                    if ptr.is_none() {
                        less_k = true;
                        break;
                    }
                    ptr = &ptr.as_ref().unwrap().next;
                }
                if less_k {
                    break;
                }
            }
            // reverse k nodes ahead
            let mut a = prev.as_mut().unwrap().next.take();
            let mut b = a.as_mut().unwrap().next.take();
            for it in 0..k - 1 {
                let new_b = b.as_mut().unwrap().next.take(); 
                b.as_mut().unwrap().next = a;
                a = b;
                b = new_b;
            }
            prev.as_mut().unwrap().next = a;
            for it in 0..k {
                prev = &mut prev.as_mut().unwrap().next;
            }
            prev.as_mut().unwrap().next = b;
        }

        dummy.unwrap().next
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;
use cargo_leet::ListNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::ListHead;

    use rstest::rstest;

    #[rstest]
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 2, ListHead::from(vec![2,1,4,3,5]).into())]
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 3, ListHead::from(vec![3,2,1,4,5]).into())]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] k: i32,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::reverse_k_group(head, k);
        assert_eq!(actual, expected);
    }
}
