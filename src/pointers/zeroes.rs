use crate::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zeroes = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                zeroes += 1;
            }
        }

        nums.retain(|n| *n != 0);
        nums.append(&mut vec![0; zeroes]);
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let mut v = [0, 1, 0, 3, 12].to_vec();
        Solution::move_zeroes(&mut v);

        assert_eq!(v, [1, 3, 12, 0, 0])
    }

    #[test]
    fn test2() {
        let mut v = [0].to_vec();
        Solution::move_zeroes(&mut v);

        assert_eq!(v, [0])
    }
}
