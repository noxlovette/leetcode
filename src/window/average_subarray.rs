use crate::Solution;
use std::ops::Div;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let average = |x: f64| x.div(k as f64);
        let mut read = 0;
        let window = &nums[read..read + k as usize];
        let mut sum: i32 = window.iter().sum();
        let mut max = average(sum as f64);

        while read < nums.len() - k as usize {
            sum = sum - nums[read] + nums[read + k as usize];
            max = max.max(average(sum as f64));
            read += 1;
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let v = Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4);

        assert_eq!(v, 12.75)
    }

    #[test]
    fn test2() {
        let v = Solution::find_max_average(vec![5], 1);
        assert_eq!(v, 5.0)
    }
}
