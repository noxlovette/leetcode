#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            traverse(&root)
        } else {
            0
        }
    }
}

fn traverse(node: &Rc<RefCell<TreeNode>>) -> i32 {
    let mut left_sum = 0;
    let mut right_sum = 0;

    if let Some(left) = node.borrow().left.as_ref() {
        left_sum += traverse(left);
    }

    if let Some(right) = node.borrow().right.as_ref() {
        right_sum += traverse(right);
    }

    left_sum.max(right_sum) + 1
}
