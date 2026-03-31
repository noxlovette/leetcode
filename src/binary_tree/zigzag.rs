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
use std::i32;
use std::rc::Rc;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut best = usize::default();

        if let Some(root) = root {
            traverse(&root, Direction::Right, 0, &mut best);
            traverse(&root, Direction::Left, 0, &mut best);
        }

        best as i32
    }
}

fn traverse(node: &Rc<RefCell<TreeNode>>, direction: Direction, current: usize, best: &mut usize) {
    let (left, right) = {
        let node_ref = node.borrow();
        (node_ref.left.clone(), node_ref.right.clone())
    };

    *best = current.max(*best);

    // we mean the previous direction
    match direction {
        Direction::Left => {
            // reset
            if let Some(left) = left.as_ref() {
                traverse(left, Direction::Left, 1, best);
            }
            // increase
            if let Some(right) = right.as_ref() {
                traverse(right, Direction::Right, current + 1, best);
            }
        }
        Direction::Right => {
            // reset
            if let Some(right) = right.as_ref() {
                traverse(right, Direction::Right, 1, best);
            }
            // increase
            if let Some(left) = left.as_ref() {
                traverse(left, Direction::Left, current + 1, best);
            }
        }
    }
}
