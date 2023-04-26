// https://leetcode.com/problems/nim-game/description/
// https://leetcode.com/problems/nim-game/submissions/936525620/

pub fn can_win_nim(n: i32) -> bool {
    n % 4 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(can_win_nim(1), true);
    }

    #[test]
    fn test2() {
        assert_eq!(can_win_nim(2), true);
    }

    #[test]
    fn test3() {
        assert_eq!(can_win_nim(3), true);
    }

    #[test]
    fn test4() {
        assert_eq!(can_win_nim(4), false);
    }

    #[test]
    fn test5() {
        assert_eq!(can_win_nim(5), true);
    }

    #[test]
    fn test6() {
        assert_eq!(can_win_nim(6), true);
    }

    #[test]
    fn test7() {
        assert_eq!(can_win_nim(7), true);
    }

    #[test]
    fn test8() {
        assert_eq!(can_win_nim(8), false);
    }

    #[test]
    fn test9() {
        assert_eq!(can_win_nim(9), true);
    }
}
