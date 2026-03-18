use crate::Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        // the window should contain at most one zero
        let mut zeroes = 0;
        let mut max = 0;
        let mut left = 0;
        let mut right = 0;

        while right < nums.len() {
            if nums[right] == 0 {
                zeroes += 1;
            }
            while zeroes > 1 {
                if nums[left] == 0 {
                    zeroes -= 1;
                }
                left += 1;
            }

            max = max.max(right - left);

            right += 1;
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let v = Solution::longest_subarray(vec![1, 1, 0, 1]);

        assert_eq!(v, 3)
    }

    #[test]
    fn test2() {
        let v = Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]);
        assert_eq!(v, 5)
    }

    #[test]
    fn test3() {
        let v = Solution::longest_subarray(vec![1, 1, 1]);
        assert_eq!(v, 2)
    }
}
