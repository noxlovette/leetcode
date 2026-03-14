use crate::Solution;

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels = s.chars().filter(|c| is_vowel(*c)).rev();
        let mut out = String::with_capacity(s.len());

        for c in s.chars() {
            if is_vowel(c)
                && let Some(v) = vowels.next()
            {
                out.push(v)
            } else {
                out.push(c);
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
        let v = Solution::reverse_vowels("IceCreAm".to_string());

        assert_eq!(v, "AceCreIm")
    }

    #[test]
    fn test2() {
        let v = Solution::reverse_vowels("leetcode".to_string());

        assert_eq!(v, "leotcede")
    }
}
