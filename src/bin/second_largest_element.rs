struct Solution;

impl Solution {
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

fn main() {
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("Second largest: {}", Solution::second_largest(nums));
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn finds_second_largest() {
        assert_eq!(Solution::second_largest(vec![3, 1, 4, 1, 5, 9, 2, 6]), 6);
    }

    #[test]
    fn handles_duplicates_of_max() {
        assert_eq!(Solution::second_largest(vec![5, 5, 3]), 3);
    }

    #[test]
    fn returns_minus_one_when_all_equal() {
        assert_eq!(Solution::second_largest(vec![7, 7, 7]), -1);
    }

    #[test]
    fn returns_minus_one_for_empty() {
        assert_eq!(Solution::second_largest(vec![]), -1);
    }
}
