// https://leetcode.com/problems/count-special-quadruplets/
// https://leetcode.com/problems/count-special-quadruplets/submissions/937502119/

pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut count = 0;
    for i in 0..len {
        for j in i + 1..len {
            for k in j + 1..len {
                for l in k + 1..len {
                    if nums[i] + nums[j] + nums[k] == nums[l] {
                        count += 1;
                    }
                }
            }
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(count_quadruplets(Vec::from([1, 2, 3, 6])), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(count_quadruplets(Vec::from([3, 3, 6, 4, 5])), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(count_quadruplets(Vec::from([1, 1, 1, 3, 5])), 4);
    }

    #[test]
    fn test4() {
        assert_eq!(count_quadruplets(Vec::from([28, 8, 49, 85, 37, 90, 20, 8])), 1);
    }

    #[test]
    fn test5() {
        assert_eq!(count_quadruplets(Vec::from([66, 95, 85, 12, 19, 59, 81, 19, 7, 71])), 0);
    }
}
