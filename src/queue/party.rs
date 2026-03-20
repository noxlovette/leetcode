use crate::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiant = VecDeque::new();
        let mut dire = VecDeque::new();
        let n = senate.len();

        for (idx, char) in senate.chars().enumerate() {
            match char {
                'D' => {
                    dire.push_back(idx);
                }
                'R' => {
                    radiant.push_back(idx);
                }
                _ => unreachable!(),
            }
        }

        while radiant.len() != 0 && dire.len() != 0 {
            let r = radiant.pop_front().unwrap();
            let d = dire.pop_front().unwrap();
            if r < d {
                radiant.push_back(r + n);
            } else {
                dire.push_back(d + n);
            }
        }

        if !radiant.is_empty() {
            return "Radiant".to_string();
        }
        return "Dire".to_string();
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let out = Solution::predict_party_victory("RD".to_string());

        assert_eq!(out, "Radiant".to_string())
    }

    #[test]
    fn test2() {
        let out = Solution::predict_party_victory("RDD".to_string());

        assert_eq!(out, "Dire".to_string())
    }

    #[test]
    fn test3() {
        let out = Solution::predict_party_victory("DDRRR".to_string());

        assert_eq!(out, "Dire".to_string())
    }
}
