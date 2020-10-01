mod tree;

use crate::tree::{TreeNode, to_tree};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }

        let mut depth = 0;
        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        deque.push_back(root);

        while !deque.is_empty() {
            let level_size = deque.len();
            depth += 1;
            for _i in 0..level_size {
                if let Some(Some(node)) = deque.pop_front() {
                    if node.borrow().left.is_some() { deque.push_back(node.borrow().left.clone()); }
                    if node.borrow().right.is_some() { deque.push_back(node.borrow().right.clone()); }
                }
            }
        }
        depth
    }
}

fn main() {
    let vec = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    println!("{}", Solution::max_depth(to_tree(vec)));
}
