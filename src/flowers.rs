use crate::Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }
        let mut mutable = flowerbed;
        let mut planted = 0;
        let len = mutable.len();
        if len == 1 {
            match mutable[0] {
                0 => {
                    if n == 1 {
                        return true;
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        for i in 0..len {
            match i {
                0 => {
                    if mutable[i] == 0 && mutable[i + 1] == 0 {
                        mutable[i] = 1;
                        planted += 1;
                    }
                }
                x if x == len - 1 => {
                    if mutable[i] == 0 && mutable[i - 1] == 0 {
                        mutable[i] = 1;
                        planted += 1;
                    }
                }
                _ => {
                    if mutable[i] == 0 && mutable[i + 1] == 0 && mutable[i - 1] == 0 {
                        mutable[i] = 1;
                        planted += 1;
                    }
                }
            }

            if planted >= n {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let v = Solution::can_place_flowers([1, 0, 0, 0, 1].to_vec(), 1);

        assert_eq!(v, true)
    }

    #[test]
    fn test2() {
        let v = Solution::can_place_flowers([1, 0, 0, 0, 1].to_vec(), 2);

        assert_eq!(v, false)
    }

    #[test]
    fn test3() {
        let v = Solution::can_place_flowers([1, 0, 0, 0, 0, 1].to_vec(), 2);

        assert_eq!(v, false)
    }

    #[test]
    fn test4() {
        let v = Solution::can_place_flowers([1, 0, 0, 0, 0, 0, 0, 0, 1].to_vec(), 3);

        assert_eq!(v, true)
    }

    #[test]
    fn test5() {
        let v = Solution::can_place_flowers([1, 0, 1, 0, 1, 0, 1].to_vec(), 1);

        assert_eq!(v, false)
    }
}
