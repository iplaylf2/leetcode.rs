pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ask = HashMap::new();

        for (index, x) in nums.iter().enumerate() {
            let expect = ask.get(x);
            match expect {
                Some(expect_index) => return vec![*expect_index, index as i32],
                None => {
                    ask.insert(target - x, index as i32);
                }
            }
        }

        return vec![];
    }
}
