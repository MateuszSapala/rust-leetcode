// https://leetcode.com/problems/rotate-image/
// https://leetcode.com/problems/rotate-image/submissions/939120496/

struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len() - 1;
        for i in 0..matrix.len() - 1 {
            for j in i..matrix.len() - i - 1 {
                let tmp = Vec::from([
                    matrix[i][j],
                    matrix[j][len - i],
                    matrix[len - i][len - j],
                    matrix[len - j][i],
                ]);
                matrix[i][j] = tmp[3];
                matrix[j][len - i] = tmp[0];
                matrix[len - i][len - j] = tmp[1];
                matrix[len - j][i] = tmp[2];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut matrix = Vec::from([Vec::from([1, 2, 3]), Vec::from([4, 5, 6]), Vec::from([7, 8, 9])]);
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, Vec::from([Vec::from([7, 4, 1]), Vec::from([8, 5, 2]), Vec::from([9, 6, 3])]));
    }

    #[test]
    fn test2() {
        let mut matrix = Vec::from([Vec::from([5, 1, 9, 11]), Vec::from([2, 4, 8, 10]), Vec::from([13, 3, 6, 7]), Vec::from([15, 14, 12, 16])]);
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, Vec::from([Vec::from([15, 13, 2, 5]), Vec::from([14, 3, 4, 1]), Vec::from([12, 6, 8, 9]), Vec::from([16, 7, 10, 11])]));
    }
}
