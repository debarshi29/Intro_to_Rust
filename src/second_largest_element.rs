struct Solution;
impl Solution{
    pub fn second_largest(nums: Vec<i32>) -> i32 {
        let mut largest = i32::MIN;
        let mut second_largest = i32::MIN;

        for &num in &nums {
            if num > largest {
                second_largest = largest;
                largest = num;
            } else if num > second_largest && num < largest {
                second_largest = num;
            }
        }

        if second_largest == i32::MIN {
            -1 // Return -1 if there is no second largest element
        } else {
            second_largest
        }
    }
}