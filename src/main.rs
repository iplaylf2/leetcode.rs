#[path = "solution/_4_median_of_two_sorted_arrays/normal.rs"]
mod solution;

mod utility;

fn main() {
    let result = solution::Solution::find_median_sorted_arrays(vec![],vec![3]);
    println!("{:#?}", result)
}
