use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/max-number-of-k-sum-pairs/?envType=study-plan-v2&envId=leetcode-75
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut out = 0;
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut sorted = nums.clone();
        sorted.sort();

        while left < right {
            match sorted[left] + sorted[right] {
                x if x == k => {
                    out += 1;
                    left += 1;
                    right -= 1;
                }
                x if x > k => {
                    right -= 1;
                }
                x if x < k => {
                    left += 1;
                }
                _ => unreachable!(),
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let v = Solution::max_operations([1, 2, 3, 4].to_vec(), 5);

        assert_eq!(v, 2)
    }

    #[test]
    fn test3() {
        let v = Solution::max_operations([3, 1, 3, 4, 3].to_vec(), 6);

        assert_eq!(v, 1)
    }
}
