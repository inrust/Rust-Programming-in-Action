mod inorder_func;

use crate::inorder_func::{traversal_by_recursive, traversal_by_stack};
use crate::inorder_func::tree::{TreeNode, to_tree};

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        return traversal_by_stack(root);
    }
}

fn main() {
    let vec = vec![Some(1), None, Some(2), Some(3)];
    println!("{:?}", Solution::inorder_traversal(to_tree(vec)));
}
