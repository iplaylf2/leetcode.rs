pub struct Solution;

use std::{cmp, mem};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let size = match s.len() {
            0 => return 0,
            1 => return 1,
            x => x,
        };

        let mut record = [-1; 127];

        let mut anchor = 0;
        let mut maximum = 0;

        for (i, x) in s.chars().enumerate() {
            let end_point = mem::replace(&mut record[x as usize], i as i32);

            if -1 != end_point {
                maximum = cmp::max(maximum, i - anchor);
                anchor = cmp::max(anchor, end_point as usize + 1);

                if size - anchor <= maximum {
                    return maximum as i32;
                }
            }
        }

        return (size - anchor) as i32;
    }
}
