// https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/
// https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/submissions/937481904/

pub fn check_ones_segment(s: String) -> bool {
    let mut one_appeared = false;
    for c in s.chars().rev() {
        if c == '1' {
            one_appeared = true;
        } else if c == '0' && one_appeared {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(check_ones_segment(String::from("1001")), false);
    }

    #[test]
    fn test2() {
        assert_eq!(check_ones_segment(String::from("110")), true);
    }
}
