// https://leetcode.com/problems/max-consecutive-ones/
// https://leetcode.com/problems/max-consecutive-ones/submissions/937071216/

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut count = 0;
    for n in nums {
        if n == 1 {
            count += 1;
            continue;
        }
        if count > max {
            max = count;
        }
        count = 0;
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
        assert_eq!(find_max_consecutive_ones(Vec::from([1, 1, 0, 1, 1, 1])), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(find_max_consecutive_ones(Vec::from([1, 0, 1, 1, 0, 1])), 2);
    }
}
