use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/greatest-common-divisor-of-strings/description/?envType=study-plan-v2&envId=leetcode-75
    pub fn gcf_of_strings(str1: String, str2: String) -> String {
        let greatest = String::new();

        loop {}
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::gcf_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC".to_string()
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::gcf_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB".to_string()
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::gcf_of_strings("AAAAAB".to_string(), "AAA".to_string()),
            "".to_string()
        )
    }

    #[test]
    fn example4() {
        assert_eq!(
            Solution::gcf_of_strings("LEET".to_string(), "CODE".to_string()),
            "".to_string()
        )
    }
}
