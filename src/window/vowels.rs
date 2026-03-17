use crate::Solution;

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn vowel_to_usize(c: char) -> usize {
    if is_vowel(c) { 1 } else { 0 }
}

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();

        let mut read = 0;

        let count = |w: &[char]| w.into_iter().filter(|c| is_vowel(**c)).count();

        let window = &chars[read..read + k as usize];

        let mut max = count(window);
        let mut current = max;

        while read < chars.len() - k as usize {
            current =
                current - vowel_to_usize(chars[read]) + vowel_to_usize(chars[read + k as usize]);
            max = max.max(current);
            read += 1;
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let v = Solution::max_vowels("abciiidef".to_string(), 4);

        assert_eq!(v, 3)
    }

    #[test]
    fn test2() {
        let v = Solution::max_vowels("aeiou".to_string(), 2);
        assert_eq!(v, 2)
    }

    #[test]
    fn test3() {
        let v = Solution::max_vowels("leetcode".to_string(), 3);
        assert_eq!(v, 2)
    }

    #[test]
    fn test4() {
        let v = Solution::max_vowels("weallloveyou".to_string(), 7);
        assert_eq!(v, 4)
    }
}
