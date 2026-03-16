use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let max = nums.len();
        let mut out = Vec::new();
        let mut left = Vec::<i32>::with_capacity(max);
        let mut right = Vec::<i32>::with_capacity(max);

        left.push(1);
        right.push(1);

        for i in 1..max {
            left.push(left[i - 1] * nums[i - 1]);
            right.push(right[i - 1] * nums[max - i]);
        }

        right.reverse();

        for i in 0..max {
            out.push(left[i] * right[i]);
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let v = Solution::product_except_self([1, 2, 3, 4].to_vec());

        assert_eq!(v, [24, 12, 8, 6])
    }

    #[test]
    fn test2() {
        let v = Solution::product_except_self([-1, 1, 0, -3, 3].to_vec());

        assert_eq!(v, [0, 0, 9, 0, 0])
    }
}
