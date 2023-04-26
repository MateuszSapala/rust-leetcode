// https://leetcode.com/problems/max-consecutive-ones/
// https://leetcode.com/problems/max-consecutive-ones/submissions/937071216/

pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(find_length_of_lcis(Vec::from([1, 3, 5, 4, 7])), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(find_length_of_lcis(Vec::from([2, 2, 2, 2, 2])), 1);
    }
}
