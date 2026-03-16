use crate::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        if t.is_empty() {
            return false;
        }
        let mut read = 0;
        let mut sub = 0;
        let s_char = s.chars().collect::<Vec<char>>();
        let t_char = t.chars().collect::<Vec<char>>();
        while read < t_char.len() {
            if s_char[sub] != t_char[read] {
                read += 1;
            } else {
                sub += 1;
                read += 1;
                if sub == s.len() {
                    return true;
                }
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
        let v = Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string());

        assert_eq!(v, true)
    }

    #[test]
    fn test2() {
        let v = Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string());

        assert_eq!(v, false)
    }

    #[test]
    fn test3() {
        let v = Solution::is_subsequence("".to_string(), "ahbgdc".to_string());

        assert_eq!(v, true)
    }

    #[test]
    fn test4() {
        let v = Solution::is_subsequence("b".to_string(), "c".to_string());

        assert_eq!(v, false)
    }
}
