use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        // first of all, if the words are different length, return false
        if word1.len() != word2.len() {
            return false;
        }

        // how to determine that no more operations are possible?

        // operation 1. swap any two existing characters. technically reorder the string freely. this technically means if two sets of the same string are equal, return true. no. that would simply mean that they contain same characters. so it is invalid without the next step
        // operation 2. change the frequency of the characters. technically a hashmap... since there is no order in a hashmap, return true if, given that sets are equal, the frequencies are equal, too

        let mut hash1 = HashMap::new();
        let mut hash2 = HashMap::new();

        for char in word1.chars() {
            hash1.entry(char).and_modify(|e| *e += 1).or_insert(1);
        }
        for char in word2.chars() {
            hash2.entry(char).and_modify(|e| *e += 1).or_insert(1);
        }

        if hash1.len() != hash2.len() {
            // operation 1
            return false;
        }

        for key in hash1.keys() {
            // operation 1
            if !hash2.contains_key(key) {
                return false;
            }
        }

        // operation 2. we need to check that the keys are the same, too
        let mut values1: Vec<i32> = hash1.into_values().collect();
        let mut values2: Vec<i32> = hash2.into_values().collect();
        values1.sort();
        values2.sort();

        values1 == values2
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let out = Solution::close_strings("abc".to_string(), "bca".to_string());

        assert_eq!(out, true)
    }

    #[test]
    fn test2() {
        let out = Solution::close_strings("a".to_string(), "aa".to_string());

        assert_eq!(out, false)
    }

    #[test]
    fn test3() {
        let out = Solution::close_strings("cabbba".to_string(), "abbccc".to_string());

        assert_eq!(out, true)
    }

    #[test]
    fn test4() {
        let out = Solution::close_strings("abbzzca".to_string(), "babzzcz".to_string());

        assert_eq!(out, false)
    }

    #[test]
    fn test5() {
        let out = Solution::close_strings(
            "aaabbbbccddeeeeefffff".to_string(),
            "aaaaabbcccdddeeeeffff".to_string(),
        );

        assert_eq!(out, false)
    }
}
