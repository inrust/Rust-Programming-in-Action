mod tree;

use crate::tree::{TreeNode, to_tree};

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                if node.borrow().left.is_none() {
                    return Self::min_depth(node.borrow().right.clone()) + 1;
                }
                if node.borrow().right.is_none() {
                    return Self::min_depth(node.borrow().left.clone()) + 1;
                }

                let left = Self::min_depth(node.borrow().left.clone());
                let right = Self::min_depth(node.borrow().right.clone());
                left.min(right) + 1
            }
            None => 0,
        }
    }
}

fn main() {
    let vec = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    println!("{}", Solution::min_depth(to_tree(vec)));
}
