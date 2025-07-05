// Box shouldn't be here probably

type ListNodePtr = Option<Box<ListNode>>;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: ListNodePtr,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_list(vals: &[i32]) -> ListNodePtr {
        let mut prev = None;
        for &val in vals.iter().rev() {
            let mut cur = Self::new(val);
            cur.next = prev;
            prev = Some(Box::new(cur));
        }
        prev
    }

    fn to_list(mut ptr: ListNodePtr) -> Vec<i32> {
        let mut vals = Vec::new();
        while !ptr.is_none() {
            vals.push(ptr.as_ref().unwrap().val);
            ptr = ptr.unwrap().next;
        }
        vals
    }
}
