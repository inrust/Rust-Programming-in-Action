pub mod tree;

use tree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

pub fn traversal_by_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    if root.is_none() { return result; }

    inorder_recursive(root, &mut result);
    result
}

fn inorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            inorder_recursive(node.borrow().left.clone(), result);
            result.push(node.borrow().val);
            inorder_recursive(node.borrow().right.clone(), result);
        }
        None => { return; }
    }
}

pub fn traversal_by_stack(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    if root.is_none() { return result; }

    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut r = root.clone();

    while r.is_some() || !stack.is_empty() {
        while let Some(node) = r {
            stack.push(node.clone());
            r = node.borrow().left.clone();
        }
        r = stack.pop();
        if let Some(node) = r {
            result.push(node.borrow().val);
            r = node.borrow().right.clone();
        }
    }
    result
}