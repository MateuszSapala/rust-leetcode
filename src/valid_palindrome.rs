// https://leetcode.com/problems/valid-palindrome/
// https://leetcode.com/problems/valid-palindrome/submissions/935946706/

pub fn is_palindrome(s: String) -> bool {
    let str: String = s.chars()
        .filter(|x| x.is_alphanumeric())
        .collect();
    let str = str.to_ascii_lowercase();
    return str.chars().eq(str.chars().rev());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama".to_owned()), true);
    }

    #[test]
    fn test2() {
        assert_eq!(is_palindrome("race a car".to_owned()), false);
    }

    #[test]
    fn test3() {
        assert_eq!(is_palindrome("0P".to_owned()), false);
    }
}
