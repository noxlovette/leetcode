use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/merge-strings-alternately/description/?envType=study-plan-v2&envId=leetcode-75
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();

        loop {
            let a = iter1.next();
            let b = iter2.next();

            if a.is_none() && b.is_none() {
                break;
            }

            if let Some(a) = a {
                result.push(a)
            }

            if let Some(b) = b {
                result.push(b)
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs".to_string()
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd".to_string()
        )
    }
}
