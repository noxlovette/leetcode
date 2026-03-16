use crate::Solution;
use std::{i32, ops::Neg};

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        if !(0..=200).contains(&s.len()) {
            return 0;
        }

        let mut chars = s.trim_start().chars().peekable();

        let negative = match chars.peek() {
            Some('+') => {
                chars.next();
                false
            }
            Some('-') => {
                chars.next();
                true
            }
            _ => false,
        };

        let r: String = chars
            .skip_while(|c| *c == '0')
            .take_while(|c| c.is_ascii_digit())
            .collect();

        let mut i = r.parse::<i64>().unwrap_or_else(|e| {
            if e.to_string().as_str() == "number too large to fit in target type" {
                i64::MAX
            } else {
                i64::default()
            }
        });

        if negative {
            i = i.neg();
        }

        i32::try_from(i).unwrap_or_else(|_| if i < 0 { i32::MIN } else { i32::MAX })
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn positive() {
        let r = Solution::my_atoi("42".to_string());
        assert_eq!(r, 42 as i32);
    }

    #[test]
    fn huge() {
        let r = Solution::my_atoi("214987128947982174982179487129847981222".to_string());
        assert_eq!(r, i32::MAX);
    }

    #[test]
    fn negative_with_whitespace() {
        let r = Solution::my_atoi(" -42".to_string());
        assert_eq!(r, -42 as i32);
    }

    #[test]
    fn non_numeric() {
        let r = Solution::my_atoi("1337c0d3".to_string());
        assert_eq!(r, 1337 as i32);
    }

    #[test]
    fn zero() {
        let r = Solution::my_atoi("0-1".to_string());
        assert_eq!(r, 0 as i32);
    }

    #[test]
    fn words_and() {
        let r = Solution::my_atoi("words and 987".to_string());
        assert_eq!(r, 0 as i32);
    }
}
