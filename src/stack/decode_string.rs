use crate::Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut out: Vec<Vec<char>> = Vec::new();

        let mut idx = 0;
        let chars: Vec<char> = s.chars().collect();

        while idx < chars.len() {
            let (read, decoded) = decode(&chars[idx..]);
            out.push(decoded);
            idx += read;
        }

        String::from_iter(out.iter().flatten())
    }
}

fn decode(chars: &[char]) -> (usize, Vec<char>) {
    let mut out = Vec::new();
    let mut idx = 0;
    let a = chars[idx];

    // found a pattern to repeat
    if let Some(mut multiplier) = a.to_digit(10) {
        while let Some(n) = chars[idx + 1].to_digit(10) {
            multiplier = multiplier * 10 + n;
            idx += 1;
        }
        // skip the [
        idx += 2;
        let mut inner = Vec::new();
        while chars[idx] != ']' {
            let (read, mut decoded) = decode(&chars[idx..]);
            idx += read;
            inner.append(&mut decoded);
        }

        for _ in 0..multiplier {
            out.append(&mut inner.clone());
        }
    } else {
        // just a regular char
        out.push(a);
    };

    (idx + 1, out)
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let out = Solution::decode_string("3[a]2[bc]".to_string());

        assert_eq!(out, "aaabcbc")
    }

    #[test]
    fn test2() {
        let out = Solution::decode_string("3[a2[c]]".to_string());

        assert_eq!(out, "accaccacc")
    }

    #[test]
    fn test3() {
        let out = Solution::decode_string("2[abc]3[cd]ef".to_string());

        assert_eq!(out, "abcabccdcdcdef")
    }

    #[test]
    fn test4() {
        let out = Solution::decode_string("100[leetcode]".to_string());

        assert_eq!(out, "leetcode".repeat(100));
    }
}
