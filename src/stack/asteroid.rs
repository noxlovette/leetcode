use crate::Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut out: Vec<i32> = Vec::new();

        let mut idx = 0;

        while idx < asteroids.len() {
            let a = asteroids[idx];

            if let Some(b) = out.last()
                && a.signum() < b.signum()
            {
                match a + b {
                    x if x < 0 => {
                        out.pop();
                    }
                    x if x > 0 => {
                        idx += 1;
                    }
                    0 => {
                        out.pop();
                        idx += 1;
                    }
                    _ => unreachable!(),
                }
            } else {
                out.push(a);
                idx += 1;
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
        let out = Solution::asteroid_collision(vec![5, 10, -5]);

        assert_eq!(out, vec![5, 10])
    }

    #[test]
    fn test2() {
        let out = Solution::asteroid_collision(vec![8, -8]);

        assert_eq!(out, vec![])
    }

    #[test]
    fn test3() {
        let out = Solution::asteroid_collision(vec![10, 2, -5]);

        assert_eq!(out, vec![10])
    }

    #[test]
    fn test4() {
        let out = Solution::asteroid_collision(vec![3, 5, -6, 2, -1, 4]);

        assert_eq!(out, vec![-6, 2, 4])
    }
}
