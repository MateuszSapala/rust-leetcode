// https://leetcode.com/problems/valid-triangle-number/
// https://leetcode.com/problems/valid-triangle-number/submissions/938388358/

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        nums.sort();
        let mut possible_triangles = 0;
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                for k in j + 1..nums.len() {
                    match (nums[i] + nums[j]).cmp(&(nums[k])) {
                        Ordering::Less | Ordering::Equal => { break; }
                        Ordering::Greater => { possible_triangles += 1 }
                    }
                }
            }
        }
        return possible_triangles;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::triangle_number(Vec::from([2, 2, 3, 4])), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::triangle_number(Vec::from([4, 2, 3, 4])), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::triangle_number(Vec::from([1])), 0);
    }
}
