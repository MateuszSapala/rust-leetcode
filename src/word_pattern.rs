// https://leetcode.com/problems/word-pattern/
// https://leetcode.com/problems/word-pattern/submissions/936845501/

use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
    if pattern.len() != s.split(" ").collect::<Vec<&str>>().len() {
        return false;
    }
    let mut map: HashMap<char, &str> = HashMap::new();
    let mut words_map: HashMap<&str, char> = HashMap::new();
    for i in 0..pattern.len() {
        let mut p = pattern.chars();
        let str = s.split(" ").collect::<Vec<&str>>();

        let p_ith = &p.nth(i).unwrap();
        let opt = map.get(p_ith);
        match opt {
            Some(val) => {
                if val != &str[i] {
                    return false;
                }
            }
            None => {
                if words_map.get(str[i]).is_some() {
                    return false;
                }
                map.insert(*p_ith, str[i]);
                words_map.insert(str[i], *p_ith);
            }
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(word_pattern("abba".to_owned(), "dog cat cat dog".to_owned()), true);
    }

    #[test]
    fn test2() {
        assert_eq!(word_pattern("abba".to_owned(), "dog cat cat fish".to_owned()), false);
    }

    #[test]
    fn test3() {
        assert_eq!(word_pattern("aaaa".to_owned(), "dog cat cat dog".to_owned()), false);
    }

    #[test]
    fn test4() {
        assert_eq!(word_pattern("abba".to_owned(), "dog dog dog dog".to_owned()), false);
    }

    #[test]
    fn test5() {
        assert_eq!(word_pattern("aaa".to_owned(), "aa aa aa aa".to_owned()), false);
    }
}
