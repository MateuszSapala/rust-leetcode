// https://leetcode.com/problems/longest-continuous-increasing-subsequence/
// https://leetcode.com/problems/longest-continuous-increasing-subsequence/submissions/937076544/

pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut count = 0;
    let mut previous = i32::MIN;
    for n in nums {
        if n > previous {
            count += 1;
            previous = n;
            continue;
        }
        previous = n;
        if count > max {
            max = count;
        }
        count = 1;
    }
    if count > max {
        max = count;
    }
    return max;
}

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

    #[test]
    fn test3() {
        assert_eq!(find_length_of_lcis(Vec::from([1, 3, 5, 4, 2, 3, 4, 5])), 4);
    }
}
