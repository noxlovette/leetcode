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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(root1) = root1
            && let Some(root2) = root2
        {
            let mut vec1 = Vec::new();
            let mut vec2 = Vec::new();

            traverse(&root1, &mut vec1);
            traverse(&root2, &mut vec2);

            vec1 == vec2
        } else {
            unreachable!()
        }
    }
}

fn traverse(node: &Rc<RefCell<TreeNode>>, vec: &mut Vec<i32>) {
    let (left, right, val) = {
        let node_ref = node.borrow();
        (node_ref.left.clone(), node_ref.right.clone(), node_ref.val)
    };

    if left.is_none() && right.is_none() {
        vec.push(val);
        return;
    }
    if let Some(left) = left.as_ref() {
        traverse(left, vec);
    }
    if let Some(right) = right.as_ref() {
        traverse(right, vec);
    }
}
