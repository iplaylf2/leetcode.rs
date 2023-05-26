#[path = "solution/_1_two_sum/normal.rs"]
mod solution;

mod utility;

fn main() {
    let result = solution::Solution::two_sum(vec![1,2,3,4,5], 3);
    println!("{:#?}", result)
}
