mod linked_list;

use crate::linked_list::{ListNode, to_list};

struct Solution;

impl Solution {
    pub fn remove_nth_from_end_1(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut cur = &mut dummy;
        let mut length = 0;

        while let Some(node) = cur.as_mut() {
            cur = &mut node.next;
            if let Some(_node) = cur { length += 1; }
        }

        let mut new_cur = dummy.as_mut();
        let idx = length - n;

        for _ in 0..idx {
            new_cur = new_cur.unwrap().next.as_mut();
        }

        let next = new_cur.as_mut().unwrap().next.as_mut().unwrap().next.take();
        new_cur.as_mut().unwrap().next = next;

        dummy.unwrap().next
    }

    pub fn remove_nth_from_end_2(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow_p = &mut dummy;
        let mut fast_p = &mut slow_p.clone();

        for _ in 1..=n + 1 {
            fast_p = &mut fast_p.as_mut().unwrap().next;
        }

        while fast_p.is_some() {
            fast_p = &mut fast_p.as_mut().unwrap().next;
            slow_p = &mut slow_p.as_mut().unwrap().next;
        }

        let next = &slow_p.as_mut().unwrap().next.as_mut().unwrap().next;
        slow_p.as_mut().unwrap().next = next.clone();

        dummy.unwrap().next
    }
}

fn main() {
    println!("{:?}", Solution::remove_nth_from_end_1(to_list(vec![1, 2, 3, 4, 5]), 2));
}
