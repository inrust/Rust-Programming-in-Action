pub mod tree;

use tree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

pub fn traversal_by_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    if root.is_none() { return result; }

    postorder_recursive(root, &mut result);
    result
}

fn postorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            postorder_recursive(node.borrow().left.clone(), result);
            postorder_recursive(node.borrow().right.clone(), result);
            result.push(node.borrow().val);
        },
        None => { return; }
    }
}

pub fn traversal_by_stack(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    if root.is_none() { return result; }

    let mut stack1: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    let mut stack2: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    stack1.push(root);

    while let Some(Some(node)) = stack1.pop() {
        if node.borrow().left.is_some() {
            stack1.push(node.borrow().left.clone());
        }
        if node.borrow().right.is_some() {
            stack1.push(node.borrow().right.clone());
        }
        stack2.push(Some(node));
    }

    while let Some(Some(node)) = stack2.pop() {
        result.push(node.borrow().val);
    }

    result
}