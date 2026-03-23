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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let val = root.borrow().val;
            traverse(&root, val)
        } else {
            unreachable!()
        }
    }
}

fn traverse(node: &Rc<RefCell<TreeNode>>, path_max: i32) -> i32 {
    let (left, right, val) = {
        let node_ref = node.borrow();
        (node_ref.left.clone(), node_ref.right.clone(), node_ref.val)
    };

    let good = if path_max <= val { 1 } else { 0 };
    let new_max = path_max.max(val);
    let mut left_count = 0;
    let mut right_count = 0;

    if let Some(left) = left.as_ref() {
        left_count = traverse(left, new_max);
    }
    if let Some(right) = right.as_ref() {
        right_count = traverse(right, new_max);
    }

    good + left_count + right_count
}
