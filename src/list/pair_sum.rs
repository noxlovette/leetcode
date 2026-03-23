use crate::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        if head.as_ref().is_none() {
            return 0;
        }

        let mut len = 0;
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            len += 1;
            cur = node.next.as_ref();
        }

        let mut head = head;
        let mut second_half = &mut head;

        for _ in 0..(len / 2) {
            // skip until the middle. the key is to have the option itself as &mut
            second_half = &mut second_half.as_mut().unwrap().next;
        }

        // own the second half
        let mut curr = second_half.take();
        let mut prev: Option<Box<ListNode>> = None;

        // inverse the second part
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        let mut curr = head.as_ref();
        let mut max = 0;

        while let Some(mut node) = prev {
            let a = curr.unwrap().as_ref().val;
            let b = node.as_ref().val;
            max = max.max(a + b);
            curr = curr.unwrap().as_ref().next.as_ref();
            prev = node.next.take();
        }

        max
    }
}
