use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut exists1 = HashSet::new();
        let mut exists2 = HashSet::new();

        for num in nums1 {
            exists1.insert(num);
        }

        for num in nums2 {
            exists2.insert(num);
        }

        vec![
            exists1.difference(&exists2).copied().collect(),
            exists2.difference(&exists1).copied().collect(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test2() {
        let out = Solution::find_difference(vec![1, 2, 2, 3], vec![1, 1, 2, 2]);

        assert_eq!(out, [vec![3], vec![]])
    }
}
