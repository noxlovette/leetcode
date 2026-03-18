use crate::Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut max = 0;
        let mut zero_count = 0;
        let mut right = 0;
        let mut left = 0;

        while right < nums.len() {
            if nums[right] == 0 {
                zero_count += 1;
            }
            while zero_count > k {
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }

            max = max.max(right as i32 - left as i32 + 1);

            right += 1;
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let v = Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2);

        assert_eq!(v, 6)
    }

    #[test]
    fn test2() {
        let v = Solution::longest_ones(
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            3,
        );
        assert_eq!(v, 10)
    }

    #[test]
    fn test3() {
        let v = Solution::longest_ones(vec![0, 0, 0, 1], 4);
        assert_eq!(v, 4)
    }
}
