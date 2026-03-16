use crate::Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut out = Vec::<bool>::new();

        for candy in candies.iter() {
            let v = candies.clone();
            out.push(v.iter().all(|c| *candy + extra_candies >= *c));
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let v = Solution::kids_with_candies([2, 3, 5, 1, 3].to_vec(), 3);

        assert_eq!(v, [true, true, true, false, true])
    }
    #[test]
    fn test2() {
        let v = Solution::kids_with_candies([4, 2, 1, 1, 2].to_vec(), 1);

        assert_eq!(v, [true, false, false, false, false])
    }
    #[test]
    fn test3() {
        let v = Solution::kids_with_candies([12, 1, 12].to_vec(), 10);

        assert_eq!(v, [true, false, true])
    }
}
