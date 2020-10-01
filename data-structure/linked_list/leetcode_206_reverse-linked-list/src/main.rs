mod linked_list;

use crate::linked_list::{ListNode, to_list};

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut curr_node) = curr.take() {
            let next_temp = curr_node.next.take();
            curr_node.next = prev.take();
            prev = Some(curr_node);
            curr = next_temp;
        }

        prev
    }
}

fn main() {
    println!("{:?}", Solution::reverse_list(to_list(vec![1, 2, 3, 4, 5])));
}
