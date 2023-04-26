// https://leetcode.com/problems/4sum/
// https://leetcode.com/problems/4sum/submissions/937550383/

use std::cmp::Ordering;
use std::collections::HashSet;

pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {
        return Vec::new();
    }
    nums.sort();
    let len = nums.len();
    let mut set = HashSet::new();
    for i in 0..len - 3 {
        if nums[i] > i32::MAX / 4 {
            continue;
        }
        for j in i + 1..len - 2 {
            let (mut k, mut l) = (j + 1, len - 1);
            while k < l {
                let result = nums[i] + nums[j] + nums[k] + nums[l];
                match result.cmp(&target) {
                    Ordering::Less => { k += 1 }
                    Ordering::Greater => { l -= 1 }
                    Ordering::Equal => {
                        set.insert(Vec::from([nums[i], nums[j], nums[k], nums[l]]));
                        k += 1;
                        l -= 1;
                    }
                }
                if result > target {} else if result > target {}
            }
        }
    }
    return set.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let expected = Vec::from([
            Vec::from([-2, -1, 1, 2]),
            Vec::from([-2, 0, 0, 2]),
            Vec::from([-1, 0, 0, 1])
        ]);
        arrays_eq(four_sum(Vec::from([1, 0, -1, 0, -2, 2]), 0), expected);
    }

    #[test]
    fn test2() {
        let expected = Vec::from([Vec::from([2, 2, 2, 2])]);
        arrays_eq(four_sum(Vec::from([2, 2, 2, 2, 2]), 8), expected);
    }

    #[test]
    fn test3() {
        let expected = Vec::from([]);
        arrays_eq(four_sum(Vec::from([0]), 0), expected);
    }

    #[test]
    fn test4() {
        let expected = Vec::from([]);
        arrays_eq(four_sum(Vec::from([1000000000, 1000000000, 1000000000, 1000000000]), -294967296), expected);
    }

    fn arrays_eq(mut result: Vec<Vec<i32>>, mut expected: Vec<Vec<i32>>) {
        assert_eq!(result.len(), expected.len());
        for i in 0..result.len() {
            result[i].sort();
            expected[i].sort();
        }
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }
}
