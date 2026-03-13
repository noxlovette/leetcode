use crate::Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if !p.contains('*') && !p.contains('.') {
            return s == p;
        }

        let mut zip = s.chars().zip(p.chars()).peekable();

        while let Some((s, p)) = zip.next() {
            if (s != p) && p != '.' {
                break;
            }

            if let Some((v, m)) = zip.peek() {
                if *m == '*' {
                    if p == '.' {
                        return true;
                    }
                    let (vec1, vec2): (Vec<_>, Vec<_>) = zip.unzip();

                    if vec1.iter().all(|el| *el == p) {
                        return true;
                    }

                    break;
                }
                continue;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn entire() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false)
    }

    #[test]
    fn asterisk() {
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true)
    }

    #[test]
    fn any() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true)
    }

    #[test]
    fn optional() {
        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b".to_string(),),
            true
        )
    }
}
