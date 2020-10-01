mod tree;

use crate::tree::{TreeNode, to_tree};

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut r = root.clone();
        while let Some(node) = r {
            if node.borrow().val == val { return Some(node); }
            if node.borrow().val > val {
                r = node.borrow().left.clone();
            } else {
                r = node.borrow().right.clone();
            }
        }
        None
    }
}
