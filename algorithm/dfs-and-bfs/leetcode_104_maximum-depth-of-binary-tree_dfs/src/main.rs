mod tree;

use crate::tree::{TreeNode, to_tree};

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left = Self::max_depth(node.borrow().left.clone());
                let right = Self::max_depth(node.borrow().right.clone());
                1 + left.max(right)
            }
            _ => 0,
        }
    }
}

fn main() {
    let vec = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    println!("{}", Solution::max_depth(to_tree(vec)));
}
