use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let side_size = grid.len();
        let mut pairs = 0;
        let mut columns: Vec<Vec<i32>> = Vec::new();

        for _ in 0..side_size {
            columns.push(Vec::new());
        }

        let mut row_hash: HashMap<Vec<i32>, i32> = HashMap::new();

        for row in grid {
            for c in 0..side_size {
                columns[c].push(row[c]);
            }
            row_hash.entry(row).and_modify(|e| *e += 1).or_insert(1);
        }

        for column in &columns {
            if let Some(key) = row_hash.get(column) {
                pairs += key;
            }
        }

        pairs
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let out = Solution::equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]);

        assert_eq!(out, 1)
    }

    #[test]
    fn test2() {
        let out = Solution::equal_pairs(vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ]);

        assert_eq!(out, 3)
    }
}
