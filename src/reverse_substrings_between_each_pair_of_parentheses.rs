// https://leetcode.com/problems/permutations/
// TODO

use std::mem::swap;

struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut result = vec![vec![]];
        for i in 0..len {
            for j in i + 1..len {}
        }
        return result;
    }

    fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
        let tmp = nums[i];
        nums[i] = nums[j];
        nums[j] = tmp;
    }

    fn generate_permutations(n: i32, vec: &Vec<i32>) -> Vec<i32> {
        if n == 1 {
            return vec;
        }
        Self::generate_permutations(n - 1, vec.clone());
        for i in 0..n {
            if n % 2 == 0 {
                Self::swap(vec.clone(), i as usize, (n - 1) as usize);
            } else {
                Self::swap(vec.clone(), 0, (n - 1) as usize);
            }
            Self::generate_permutations(n-1, vec.clone());
        }
        return Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let permutations = Solution::permute(Vec::from([1, 2, 3]));
        let expected = Vec::from([
            Vec::from([1, 2, 3]),
            Vec::from([1, 3, 2]),
            Vec::from([2, 1, 3]),
            Vec::from([2, 3, 1]),
            Vec::from([3, 1, 2]),
            Vec::from([3, 2, 1]),
        ]);
        verify(permutations, expected);
    }

    #[test]
    fn test2() {
        let permutations = Solution::permute(Vec::from([0, 1]));
        let expected = Vec::from([
            Vec::from([0, 1]),
            Vec::from([1, 0]),
        ]);
        verify(permutations, expected);
    }

    #[test]
    fn test3() {
        let permutations = Solution::permute(Vec::from([1]));
        let expected = Vec::from([Vec::from([1]), ]);
        verify(permutations, expected);
    }

    fn verify(permutations: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
        'v_loop: for v in &expected {
            for p in &permutations {
                if v.eq(p) {
                    continue 'v_loop;
                }
            }
            println!("{:?} {:?}", permutations, expected);
            panic!("{:?} not found", v)
        }
    }
}
