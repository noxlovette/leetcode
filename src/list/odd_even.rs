use crate::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_ref().is_none() {
            return None;
        }

        if head.as_ref().and_then(|n| n.next.as_ref()).is_none() {
            return head;
        }

        let mut odd_dummy = Box::new(ListNode::new(0));
        let mut even_dummy = Box::new(ListNode::new(0));

        let mut odd_tail = &mut odd_dummy;
        let mut even_tail = &mut even_dummy;

        let mut is_odd = true;

        while let Some(mut node) = head {
            head = node.next.take();

            if is_odd {
                odd_tail.next = Some(node);
                odd_tail = odd_tail.next.as_mut().unwrap();
            } else {
                even_tail.next = Some(node);
                even_tail = even_tail.next.as_mut().unwrap();
            }
            is_odd = !is_odd
        }

        odd_tail.next = even_dummy.next;
        odd_dummy.next
    }
}
