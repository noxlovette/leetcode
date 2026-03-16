use crate::Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut first, mut second) = (i32::MAX, i32::MAX);

        for n in nums {
            if n <= first {
                first = n
            } else if n <= second && n > first {
                second = n
            } else if n > second {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let v = Solution::increasing_triplet([1, 2, 3, 4, 5].to_vec());

        assert_eq!(v, true)
    }

    #[test]
    fn test2() {
        let v = Solution::increasing_triplet([5, 4, 3, 2, 1].to_vec());

        assert_eq!(v, false)
    }

    #[test]
    fn test3() {
        let v = Solution::increasing_triplet([2, 1, 5, 0, 4, 6].to_vec());

        assert_eq!(v, true)
    }
}
