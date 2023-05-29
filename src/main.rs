#[path = "solution/_3_longest_substring_without_repeating_characters/normal.rs"]
mod solution;

mod utility;

fn main() {
    let result = solution::Solution::length_of_longest_substring(String::from("111"));
    println!("{:#?}", result)
}
