use crate::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }
        let mut res = i32::default();
        let mut init = x;

        while init != 0 {
            res = res * 10 + (init % 10);

            if let Some(r) = init.checked_div(10) {
                init = r;
            } else {
                break;
            }
        }

        res == x
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true)
    }
    #[test]
    fn not_palindrome() {
        assert_eq!(Solution::is_palindrome(-121), false)
    }

    #[test]
    fn not_palindrome_again() {
        assert_eq!(Solution::is_palindrome(10), false)
    }
}
