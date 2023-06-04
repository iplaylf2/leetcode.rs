pub struct Solution;

use std::cmp;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();

        if 0 == len1 && 0 == len2 {
            return 0.0;
        }

        let (nums1, nums2) = if len1 <= len2 {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };

        let len1 = nums1.len();
        let len2 = nums2.len();

        let len_all = len1 + len2;
        let is_two = len_all % 2 == 0;

        let mut j1 = len1 as i32;
        let mut j2 = ((len_all + 1) / 2 - len1) as i32;

        let mut j1_begin = 0;
        let mut j1_end: i32 = j1;

        loop {
            let left1 = match j1 - 1 {
                i if i < 0 => i32::MIN,
                i => nums1[i as usize],
            };

            let right2 = match j2 as usize {
                i if len2 == i => i32::MAX,
                i => nums2[i],
            };

            if left1 <= right2 {
                let left2 = match j2 - 1 {
                    i if i < 0 => i32::MIN,
                    i => nums2[i as usize],
                };

                let right1 = match j1 as usize {
                    i if len1 == i => i32::MAX,
                    i => nums1[i],
                };

                if left2 <= right1 {
                    return if is_two {
                        let mid1 = cmp::max(left1, left2);
                        let mid2 = cmp::min(right1, right2);
                        (mid1 + mid2) as f64 / 2.0
                    } else {
                        cmp::max(left1, left2) as f64
                    };
                } else {
                    j1_begin = j1;
                }
            } else {
                j1_end = j1;
            }

            let j1_old = j1;
            j1 = (j1_begin + j1_end) / 2;
            j2 = j2 - (j1 - j1_old);
        }
    }
}
