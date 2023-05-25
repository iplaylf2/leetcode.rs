pub struct Solution;

use crate::utility::list_node::ListNode;

use std::{mem, ops::DerefMut};

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;

        let mut prefix = ListNode::new(0);
        let mut final_node = &mut prefix;

        let mut l1 = l1;
        let mut l2 = l2;

        loop {
            let mut l1_box = l1.unwrap();

            let ListNode {
                val: l1_val,
                next: l1_next,
            } = l1_box.deref_mut();

            let ListNode {
                val: l2_val,
                next: l2_next,
            } = l2.as_mut().unwrap().deref_mut();

            let x = *l1_val + *l2_val + carry;

            *l1_val = if x < 10 {
                carry = 0;
                x
            } else {
                carry = 1;
                x - 10
            };

            l1 = mem::take(l1_next);
            l2 = mem::take(l2_next);

            final_node.next = Some(l1_box);
            final_node = final_node.next.as_mut().unwrap();

            if l1.is_none() {
                l1 = l2;
                break;
            }

            if l2.is_none() {
                break;
            }
        }

        while let Some(mut l1_box) = l1 {
            let ListNode { val, next: l1_next } = l1_box.as_mut();

            let x = *val + carry;

            if x < 10 {
                *val = x;
                
                final_node.next = Some(l1_box);
                return prefix.next;
            } else {
                carry = 1;

                *val = 0;
                l1 = mem::take(l1_next);

                final_node.next = Some(l1_box);
                final_node = final_node.next.as_mut().unwrap();
            }
        }

        if 1 == carry {
            final_node.next = Some(Box::new(ListNode::new(1)));
        }

        prefix.next
    }
}
