#[path = "solution/_1_two_sum/normal.rs"]
mod solution;
fn main() {
    let result = solution::Solution::two_sum(vec![2,7,11,15], 9);
    println!("{:#?}", result)
}
