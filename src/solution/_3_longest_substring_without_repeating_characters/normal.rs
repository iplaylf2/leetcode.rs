pub struct Solution;

use std::{cmp, mem};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let size = match s.len() {
            0 => return 0,
            1 => return 1,
            x => x,
        } as i32;

        let mut record = [-1; 127];

        let mut anchor: i32 = 0;
        let mut maximum: i32 = 0;

        for (i, x) in s.chars().enumerate() {
            let current: i32 = i as i32;
            let end_point = mem::replace(&mut record[x as usize], current);

            if -1 != end_point {
                maximum = cmp::max(maximum, current - anchor);
                anchor = cmp::max(anchor, end_point + 1);

                if size - anchor <= maximum {
                    return maximum;
                }
            }
        }

        return size - anchor;
    }
}
