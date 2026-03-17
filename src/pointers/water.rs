use crate::Solution;

impl Solution {
    pub fn max_area(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let area = |l: usize, r: usize| nums[l].min(nums[r]) * (r - l) as i32;
        let mut best = area(left, right);

        while left != right {
            if area(left, right) > best {
                best = area(left, right)
            }
            if nums[left] <= nums[right] {
                left += 1;
                continue;
            }
            if nums[right] < nums[left] {
                right -= 1;
                continue;
            }
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let v = Solution::max_area([1, 8, 6, 2, 5, 4, 8, 3, 7].to_vec());

        assert_eq!(v, 49)
    }
}

#[test]
fn test3() {
    let v = Solution::max_area([1, 2, 1].to_vec());

    assert_eq!(v, 2)
}

#[test]
fn test2() {
    let v = Solution::max_area([1, 1].to_vec());

    assert_eq!(v, 1)
}

#[test]
fn test4() {
    let v = Solution::max_area([1, 2].to_vec());

    assert_eq!(v, 1)
}
