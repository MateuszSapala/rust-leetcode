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

// pub fn is_palindrome(s: String) -> bool {
//     let str:String = s.to_lowercase().chars().map(|x| if x.is_alphanumeric() { x } else { ' ' }).collect();
//     let str = str.replace(" ", "");
//     let last_idx = str.len() - 1;
//     for i in 0..(str.len() / 2) {
//         if str.chars().nth(i) != str.chars().nth(last_idx - i) {
//             return false;
//         }
//     }
//     return true;
// }

// pub fn is_palindrome(s: String) -> bool {
//     let mut vec_chr = Vec::new();
//     for i in 0..s.len(){
//         let c = s.chars().nth(i).expect("AAAAAAAAAAAAAAAAa");
//         if c.is_alphanumeric() {
//             vec_chr.push(c);
//         }
//     }
//     let str: String = vec_chr.iter().collect();
//     let str = str.to_lowercase();
//     let last_idx = str.len() - 1;
//     for i in 0..(str.len() / 2) {
//         if str.chars().nth(i) != str.chars().nth(last_idx - i) {
//             return false;
//         }
//     }
//     return true;
// }
