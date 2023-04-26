// https://leetcode.com/problems/valid-parenthesis-string/
// https://leetcode.com/problems/valid-parenthesis-string/submissions/937858080/

use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut open_brackets, mut close_brackets, mut wildcards) = (0, 0, 0);
        for c in s.chars() {
            match c {
                '(' => { open_brackets += 1 }
                ')' => { close_brackets += 1 }
                _ => { wildcards += 1 }
            }
        }
        let (min, max) = (min(open_brackets, close_brackets), max(open_brackets, close_brackets));
        if min + wildcards < max {
            return false;
        }

        let (wildcards_to_open, wildcards_to_close) = (max - open_brackets, max - close_brackets);
        let wildcards_both = (wildcards - wildcards_to_open - wildcards_to_close) / 2;
        let (mut wildcards_to_open, mut wildcards_to_close) = (wildcards_to_open + wildcards_both, wildcards_to_close + wildcards_both);

        let mut opened = 0;
        for c in s.chars() {
            match c {
                '(' => {
                    opened += 1;
                }
                ')' => {
                    opened -= 1;
                    if opened < 0 {
                        return false;
                    }
                }
                _ => {
                    wildcards -= 1;
                    if wildcards_to_open > 0 {
                        opened += 1;
                        wildcards_to_open -= 1;
                        continue;
                    }
                    if wildcards_to_close > wildcards {
                        wildcards_to_close -= 1;
                        opened -= 1;
                        if opened < 0 {
                            return false;
                        }
                        continue;
                    }
                }
            }
        }
        return opened == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::check_valid_string(String::from("()")), true);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::check_valid_string(String::from("(*)")), true);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::check_valid_string(String::from("(*))")), true);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::check_valid_string(String::from("(**))")), true);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::check_valid_string(String::from("((**)")), true);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::check_valid_string(String::from("(*)))")), false);
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::check_valid_string(String::from("())***)")), false);
    }

    #[test]
    fn test8() {
        assert_eq!(Solution::check_valid_string(String::from("(()((()***)")), true);
    }

    #[test]
    fn test9() {
        assert_eq!(Solution::check_valid_string(String::from("((((()(()()()*()(((((*)()*(**(())))))(())()())(((())())())))))))(((((())*)))()))(()((*()*(*)))(*)()")), true);
    }
}
