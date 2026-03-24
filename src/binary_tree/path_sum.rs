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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut path: HashMap<i64, i32> = HashMap::new();
        path.insert(0, 1);

        if let Some(root) = root {
            let current_sum = 0;
            traverse(&root, &mut path, target_sum as i64, current_sum)
        } else {
            0
        }
    }
}

fn traverse(
    node: &Rc<RefCell<TreeNode>>,
    path: &mut HashMap<i64, i32>,
    target: i64,
    parent: i64,
) -> i32 {
    let mut total_paths = 0;
    let (left, right, val) = {
        let node_ref = node.borrow();
        (
            node_ref.left.clone(),
            node_ref.right.clone(),
            node_ref.val.clone(),
        )
    };
    let curr = val;
    let child_sum = parent + curr as i64;

    if let Some(&v) = path.get(&(child_sum - target)) {
        total_paths += v;
    }

    path.entry(child_sum).and_modify(|e| *e += 1).or_insert(1);

    if let Some(left) = left.as_ref() {
        total_paths += traverse(left, path, target, child_sum);
    }

    if let Some(right) = right.as_ref() {
        total_paths += traverse(right, path, target, child_sum);
    }

    path.entry(child_sum).and_modify(|e| *e -= 1);

    total_paths
}
