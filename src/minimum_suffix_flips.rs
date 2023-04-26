// https://leetcode.com/problems/minimum-suffix-flips/
// https://leetcode.com/problems/minimum-suffix-flips/submissions/938394976/

struct Solution {}

impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut flips = 0;
        let mut previous = '0';
        for c in target.chars() {
            if c != previous {
                flips += 1;
                previous = c;
            }
        }
        return flips;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_flips(String::from("10111")), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_flips(String::from("101")), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_flips(String::from("00000")), 0);
    }
}
