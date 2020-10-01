mod tree;

use crate::tree::{TreeNode, to_tree};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut levels: Vec<Vec<i32>> = vec![];
        if root.is_none() { return levels; }

        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        deque.push_back(root);

        while !deque.is_empty() {
            let mut current_level = vec![];

            let level_length = deque.len();
            for _ in 0..level_length {
                let n = deque.pop_front();
                if let Some(Some(node)) = n {
                    current_level.push(node.borrow().val);

                    if node.borrow().left.is_some() { deque.push_back(node.borrow().left.clone()); }
                    if node.borrow().right.is_some() { deque.push_back(node.borrow().right.clone()); }
                }
            }

            levels.push(current_level);
        }

        levels
    }
}

fn main() {
    let vec = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    println!("{:?}", Solution::level_order(to_tree(vec)));
}
