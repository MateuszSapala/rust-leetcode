// https://leetcode.com/problems/maximum-number-of-consecutive-values-you-can-make/
// TODO

struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_maximum_consecutive(Vec::from([1,3])), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::get_maximum_consecutive(Vec::from([1,1,1,4])), 8);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::get_maximum_consecutive(Vec::from([1,3])), 2);
    }
}
