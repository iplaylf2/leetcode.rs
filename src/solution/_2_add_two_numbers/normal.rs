pub struct Solution;

use crate::utility::list_node::ListNode;

use std::{mem, ops::DerefMut};

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = l1;

        let (current, carry) = Self::braid(head.as_mut().unwrap(), l2.as_mut().unwrap());

        Self::smooth_out(current, carry);

        return head;
    }

    fn braid<'a>(
        mut l1: &'a mut Box<ListNode>,
        mut l2: &'a mut Box<ListNode>,
    ) -> (&'a mut Option<Box<ListNode>>, i32) {
        let mut carry = 0;

        loop {
            let ListNode {
                val: l1_val,
                next: l1_next,
            } = l1.deref_mut();

            let ListNode {
                val: l2_val,
                next: l2_next,
            } = l2.deref_mut();

            {
                let (x, c) = match *l1_val + *l2_val + carry {
                    x if x < 10 => (x, 0),
                    x => (x - 10, 1),
                };
                *l1_val = x;
                carry = c;
            }

            l1 = match l1_next.as_ref() {
                Some(next) => unsafe {
                    let next = next as *const Box<ListNode> as *mut Box<ListNode>;
                    &mut *next
                },
                None => {
                    *l1_next = mem::take(l2_next);
                    return (l1_next, carry);
                }
            };

            l2 = match l2_next {
                Some(next) => next,
                None => {
                    return (l1_next, carry);
                }
            };
        }
    }

    fn smooth_out(mut current: &mut Option<Box<ListNode>>, carry: i32) {
        if 0 == carry {
            return;
        }

        loop {
            current = match current {
                Some(node) => {
                    let ListNode { val, next } = node.deref_mut();

                    match *val + 1 {
                        x if x < 10 => {
                            *val = x;
                            break;
                        }
                        _ => {
                            *val = 0;
                            next
                        }
                    }
                }
                None => {
                    *current = Some(Box::new(ListNode::new(1)));
                    break;
                }
            }
        }
    }
}
