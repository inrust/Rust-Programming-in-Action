mod tree;

use crate::tree::{TreeNode, to_tree};

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() { return Some(Rc::new(RefCell::new(TreeNode::new(val)))); }
        insert(&root, val);
        root
    }
}

fn insert(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
    if let Some(node) = root {
        let mut n = node.borrow_mut();
        let target = if val > n.val { &mut n.right } else { &mut n.left };

        if target.is_some() {
            return insert(target, val);
        }

        *target = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
}
