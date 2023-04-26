// https://leetcode.com/problems/decode-string/
// https://leetcode.com/problems/decode-string/submissions/937839741/

struct Solution {}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut result = String::new();
        let mut number = 0;
        let mut braces = 0;
        let mut pattern = String::new();
        for c in s.chars() {
            match c {
                '0'..='9' => {
                    if braces == 0 {
                        number = number * 10 + c.to_digit(10).unwrap();
                        continue;
                    }
                    pattern.push(c);
                }
                '[' => {
                    if braces != 0 {
                        pattern.push(c);
                    }
                    braces += 1;
                }
                ']' => {
                    braces -= 1;
                    if braces != 0 {
                        pattern.push(c);
                        continue;
                    }
                    let split: Vec<&str> = pattern.split("]").collect();
                    if split.len() != 1 {
                        pattern = Self::decode_string(pattern.clone());
                    }
                    for _ in 0..number {
                        result.push_str(&pattern);
                    }
                    number = 0;
                    pattern = String::new();
                }
                'a'..='z' => {
                    if number == 0 {
                        result.push(c);
                        continue;
                    }
                    pattern.push(c);
                }
                _ => {}
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::decode_string(String::from("3[a]2[bc]")), "aaabcbc");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::decode_string(String::from("3[a2[c]]")), "accaccacc");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::decode_string(String::from("2[abc]3[cd]ef")), "abcabccdcdcdef");
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::decode_string(String::from("3[a]2[c]")), "aaacc");
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::decode_string(String::from("3[a]")), "aaa");
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::decode_string(String::from("3[a]ac")), "aaaac");
    }
}
