use crate::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_ref().and_then(|n| n.next.as_ref()).is_none() {
            return None;
        }
        let mut len = 0;
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            len += 1;
            cur = node.next.as_ref()
        }

        let target = len / 2;
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy;

        for _ in 0..target {
            cur = cur.next.as_mut().unwrap()
        }

        cur.next = cur.next.as_mut().unwrap().next.take();

        dummy.next
    }
}
