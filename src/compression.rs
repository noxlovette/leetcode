use crate::Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() == 1 {
            return 1;
        }
        let mut count = 0;
        let mut group_start = 0;
        let mut read = 0;
        while read < chars.len() {
            if chars[read] == chars[group_start] {
                count += 1;
                read += 1;
            } else {
                let mut ins = [chars[group_start]].to_vec();
                if count > 1 {
                    ins.append(&mut count.to_string().chars().collect());
                }
                let write = ins.len();
                chars.splice(group_start..read, ins);
                group_start += write;
                read = group_start;
                count = 0;
            }
        }
        let mut ins = [chars[group_start]].to_vec();
        if count > 1 {
            ins.append(&mut count.to_string().chars().collect());
        }
        chars.splice(group_start..read, ins);

        chars.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let v = Solution::compress(&mut ['a', 'a', 'b', 'b', 'c', 'c', 'c'].to_vec());

        assert_eq!(v, 6)
    }

    #[test]
    fn test2() {
        let v = Solution::compress(&mut ['a'].to_vec());

        assert_eq!(v, 1)
    }

    #[test]
    fn test3() {
        let v = Solution::compress(
            &mut [
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
            ]
            .to_vec(),
        );

        assert_eq!(v, 4)
    }
}
