#[path = "solution/_2_add_two_numbers/normal.rs"]
mod solution;

mod utility;

use crate::utility::list_node::ListNode;

fn main() {
    let result = solution::Solution::add_two_numbers(
        Some(Box::new(ListNode::new(1))),
        Some(Box::new(ListNode::new(1))),
    );
    println!("{:#?}", result)
}
