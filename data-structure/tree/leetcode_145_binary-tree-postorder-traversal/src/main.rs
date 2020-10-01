mod postorder_func;

use crate::postorder_func::{traversal_by_recursive, traversal_by_stack};
use crate::postorder_func::tree::{TreeNode, to_tree};

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        return traversal_by_recursive(root);
    }
}

fn main() {
    let vec = vec![Some(1), None, Some(2), Some(3)];
    println!("{:?}", Solution::postorder_traversal(to_tree(vec)));
}
