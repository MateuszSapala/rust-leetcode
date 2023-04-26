// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/
// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/submissions/939493276/

struct Solution {}

impl Solution {
    pub fn reverse_parentheses(mut s: String) -> String {
        let mut i = 0;
        if s.len() == 0 {
            return s;
        }
        while i < s.len() - 1 {
            if s.chars().nth(i).unwrap() == '(' {
                break;
            }
            i += 1;
        }
        if i == s.len() - 1 {
            return s;
        }

        let mut j = i + 1;
        let mut opened = 1;
        while j < s.len() - 1 {
            if s.chars().nth(j).unwrap() == ')' {
                opened -= 1;
                if opened == 0 {
                    break;
                }
            } else if s.chars().nth(j).unwrap() == '(' {
                opened += 1;
            }
            j += 1;
        }
        if j - i == 1 {
            s.remove(j);
            s.remove(i);
            return Self::reverse_parentheses(s);
        }
        let (rest, end) = s.split_at(j);
        let mut end = end.to_string();
        end.remove(0); //remove )
        let (start, mid) = rest.split_at(i);
        let mut mid = mid.to_string();
        mid.remove(0); //remove (
        mid = Self::reverse_parentheses(mid);
        mid = mid.chars().rev().collect(); //reverse
        return Self::reverse_parentheses(start.to_string() + &mid + &end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse_parentheses(String::from("(abcd)")), "dcba");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::reverse_parentheses(String::from("(u(love)i)")), "iloveu");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::reverse_parentheses(String::from("(ed(et(oc))el)")), "leetcode");
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::reverse_parentheses(String::from("vdgzyj()")), "vdgzyj");
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::reverse_parentheses(String::from("ta()usw((((a))))")), "tauswa");
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::reverse_parentheses(String::from("((((a))))")), "a");
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::reverse_parentheses(String::from("sxmdll(q)eki(x)")), "sxmdllqekix");
    }

    #[test]
    fn test8() {
        assert_eq!(Solution::reverse_parentheses(String::from("eki(x)")), "ekix");
    }

    #[test]
    fn test9() {
        assert_eq!(Solution::reverse_parentheses(String::from("yhqqvjhjchlahdn(())")), "yhqqvjhjchlahdn");
    }

    #[test]
    fn test10() {
        assert_eq!(Solution::reverse_parentheses(String::from("(())")), "");
    }
}

// impl Solution {
//     pub fn reverse_parentheses(mut s: String) -> String {
//         let (mut i, mut j) = (0, s.len() - 1);
//         while i < j {
//             if s.chars().nth(i).unwrap() == '(' {
//                 break;
//             }
//             i += 1;
//         }
//         if i == j {
//             return s;
//         }
//         while j > 0 {
//             if s.chars().nth(j).unwrap() == ')' {
//                 break;
//             }
//             j -= 1;
//         }
//         if j - i == 1 {
//             s.remove(j);
//             s.remove(i);
//             return s;
//         }
//         let (rest, end) = s.split_at(j);
//         let mut end = end.to_string();
//         end.remove(0); //remove )
//         let (start, mid) = rest.split_at(i);
//         let mut mid = mid.to_string();
//         mid.remove(0); //remove (
//         mid = Self::reverse_parentheses(mid);
//         mid = mid.chars().rev().collect(); //reverse
//         return start.to_string() + &mid + &end;
//     }
// }
