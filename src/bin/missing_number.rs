struct Solution;

impl Solution {
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n: i32 = nums.len() as i32;
    let expected_sum: i32 = n * (n + 1) / 2;
    let actual_sum: i32 = nums.iter().sum();
    expected_sum - actual_sum
}
}
fn main() {
    let nums = vec![3, 0, 1];
    println!("Missing number: {}", Solution::missing_number(nums));
}