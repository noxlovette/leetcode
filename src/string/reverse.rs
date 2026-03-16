use crate::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut phrase = Vec::<String>::new();
        let mut word = String::new();

        for c in s.chars() {
            if c.is_ascii_alphanumeric() {
                word.push(c);
            } else if c.is_whitespace() {
                if word.len() > 0 {
                    phrase.push(word.drain(..).collect());
                }
            }
        }
        if word.len() > 0 {
            phrase.push(word.drain(..).collect());
        }

        phrase.reverse();
        phrase.join(" ").trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let v = Solution::reverse_words("the sky is blue".to_string());

        assert_eq!(v, "blue is sky the")
    }

    #[test]
    fn test2() {
        let v = Solution::reverse_words("   hello world  ".to_string());

        assert_eq!(v, "world hello")
    }

    #[test]
    fn test3() {
        let v = Solution::reverse_words("a good     example".to_string());

        assert_eq!(v, "example good a")
    }
}
