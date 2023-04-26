// https://leetcode.com/problems/equal-sum-arrays-with-minimum-number-of-operations/
// https://leetcode.com/problems/equal-sum-arrays-with-minimum-number-of-operations/submissions/938390116/

use std::cmp::{min, Ordering};

struct Solution {}

impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        if nums1.len() * 6 < nums2.len() || nums2.len() * 6 < nums1.len() {
            return -1;
        }
        let (mut n1, mut n1_sum) = Self::vec_sum(nums1);
        let (mut n2, mut n2_sum) = Self::vec_sum(nums2);
        let mut operations = 0;
        loop {
            match n1_sum.cmp(&n2_sum) {
                Ordering::Equal => { return operations; }
                Ordering::Less => {
                    let (n, n_sum) = (n1.clone(), n1_sum);
                    n1 = n2.clone();
                    n1_sum = n2_sum;
                    n2 = n;
                    n2_sum = n_sum;
                }
                Ordering::Greater => {
                    for diff in (0..min(n1_sum - n2_sum + 1, 6)).rev() {
                        if Self::subtract_from_sum(&mut n1, &mut n1_sum, diff) {
                            operations += 1;
                            break;
                        } else if Self::subtract_from_sum(&mut n2, &mut n2_sum, -diff) {
                            operations += 1;
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn vec_sum(nums: Vec<i32>) -> (Vec<i32>, i32) {
        let mut n: Vec<i32> = Vec::from([0, 0, 0, 0, 0, 0, 0]);
        let mut n_sum = 0;
        for val in nums {
            n[val as usize] += 1;
            n_sum += val;
        }
        return (n, n_sum);
    }

    pub fn subtract_from_sum(nums: &mut Vec<i32>, nums_sum: &mut i32, diff: i32) -> bool {
        let mut remove_idx = if diff > 0 { 6 } else { 6 + diff };
        let mut add_idx = if diff > 0 { 6 - diff } else { 6 };
        while add_idx > 0 && remove_idx > 0 {
            if nums[remove_idx as usize] > 0 {
                nums[remove_idx as usize] -= 1;
                nums[add_idx as usize] += 1;
                *nums_sum = *nums_sum - diff;
                return true;
            }
            remove_idx -= 1;
            add_idx -= 1;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_operations(Vec::from([1, 2, 3, 4, 5, 6]), Vec::from([1, 1, 2, 2, 2, 2])), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_operations(Vec::from([1, 1, 1, 1, 1, 1, 1]), Vec::from([6])), -1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_operations(Vec::from([6, 6]), Vec::from([1])), 3);
    }

    #[test]
    fn test_update_vec_add() {
        let mut vec = Vec::from([0, 1, 1, 0, 0, 0, 0]);
        let mut sum = 3;
        let returned = Solution::subtract_from_sum(&mut vec, &mut sum, 1);
        assert_eq!(returned, true);
        assert_eq!(vec, Vec::from([0, 2, 0, 0, 0, 0, 0]));
        assert_eq!(sum, 2);
    }

    #[test]
    fn test_update_vec_subtract() {
        let mut vec = Vec::from([0, 1, 1, 0, 0, 0, 0]);
        let mut sum = 3;
        let returned = Solution::subtract_from_sum(&mut vec, &mut sum, -1);
        assert_eq!(returned, true);
        assert_eq!(vec, Vec::from([0, 1, 0, 1, 0, 0, 0]));
        assert_eq!(sum, 4);
    }
}
