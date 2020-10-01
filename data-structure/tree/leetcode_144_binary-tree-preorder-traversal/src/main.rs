mod preorder_func;

use crate::preorder_func::{traversal_by_recursive, traversal_by_stack};
use crate::preorder_func::tree::{TreeNode, to_tree};

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        return traversal_by_stack(root);
    }
}

fn main() {
    let vec = vec![Some(1), None, Some(2), Some(3)];
    println!("{:?}", Solution::preorder_traversal(to_tree(vec)));
}
