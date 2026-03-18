use crate::Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        // the sum of all the numbers to the left of idx
        let mut sum_left: Vec<i32> = vec![0];

        // the sum of all the numbers to the right of idx
        let mut sum_right: Vec<i32> = vec![0];

        let last = nums.len() - 1;

        for i in 0..last {
            sum_left.push(sum_left[i] + nums[i]);
            sum_right.push(sum_right[i] + nums[last - i]);
        }

        sum_right.reverse();

        // the answer will be the the idx where sum_left[idx] == sum_right[idx]
        let idx = if let Some(idx) = sum_left
            .iter()
            .zip(sum_right.iter())
            .position(|(l, r)| l == r)
        {
            idx as i32
        } else {
            -1
        };

        idx
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let out = Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]);

        assert_eq!(out, 3)
    }

    #[test]
    fn test2() {
        let out = Solution::pivot_index(vec![1, 2, 3]);

        assert_eq!(out, -1)
    }

    #[test]
    fn test3() {
        let out = Solution::pivot_index(vec![2, 1, -1]);

        assert_eq!(out, 0)
    }
}
