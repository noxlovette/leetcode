use crate::Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut out = String::new();
        let chars: Vec<char> = s.chars().collect();
        let mut ptr = 0;
        while ptr < s.len() {
            let char = chars[ptr];
            if char == '*' {
                out.pop();
            } else {
                out.push(char);
            }
            ptr += 1;
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let out = Solution::remove_stars("leet**cod*e".to_string());

        assert_eq!(out, "lecoe".to_string())
    }

    #[test]
    fn test2() {
        let out = Solution::remove_stars("erase*****".to_string());

        assert_eq!(out, "".to_string())
    }
}
