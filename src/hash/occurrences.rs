use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut hash = HashMap::new();

        for a in arr {
            hash.entry(a).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut v: Vec<i32> = hash.into_values().collect();

        v.sort();

        let len = v.len();

        v.dedup();

        if v.len() == len {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let out = Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]);

        assert_eq!(out, true)
    }

    #[test]
    fn test2() {
        let out = Solution::unique_occurrences(vec![1, 2]);

        assert_eq!(out, false)
    }

    #[test]
    fn test3() {
        let out = Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]);

        assert_eq!(out, true)
    }
}
