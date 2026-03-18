use crate::Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut altitude = vec![0];
        let mut idx = 0;

        while idx < gain.len() {
            altitude.push(altitude[idx] + gain[idx]);
            idx += 1;
        }

        altitude.into_iter().max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let out = Solution::largest_altitude(vec![-5, 1, 5, 0, -7]);

        assert_eq!(out, 1)
    }

    #[test]
    fn test2() {
        let out = Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]);

        assert_eq!(out, 0)
    }
}
