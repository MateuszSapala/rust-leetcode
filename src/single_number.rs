// https://leetcode.com/problems/single-number/
// https://leetcode.com/problems/single-number/submissions/936521390/

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut nums = nums.clone();
    loop {
        let x = nums.pop().expect("No unique value");
        let len = nums.len();
        nums.retain(|a| *a != x);
        if len == nums.len() {
            return x;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(single_number(Vec::from([2, 2, 1])), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(single_number(Vec::from([4, 1, 2, 1, 2])), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(single_number(Vec::from([1])), 1);
    }
}
