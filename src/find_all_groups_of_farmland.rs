// https://leetcode.com/problems/find-all-groups-of-farmland/
// https://leetcode.com/problems/find-all-groups-of-farmland/submissions/937576845/

use std::cmp::min;

const FOREST: i32 = 0;
const FARMLAND: i32 = 1;

pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let width = land.len();
    if width == 0 {
        return Vec::new();
    }
    let height = land[0].len();
    let mut result = Vec::new();
    for i in 0..width {
        for j in 0..height {
            if land[i][j] == FOREST {
                continue;
            }
            if i > 0 && land[i - 1][j] == FARMLAND {
                continue;
            }
            if j > 0 && land[i][j - 1] == FARMLAND {
                continue;
            }
            let mut k = i;
            for x in i..width {
                if land[x][j] == FARMLAND {
                    k = x;
                    continue;
                }
                break;
            }
            let mut l = i;
            for x in j..height {
                if land[i][x] == FARMLAND {
                    l = x;
                    continue;
                }
                break;
            }
            result.push(Vec::from([i as i32, j as i32, k as i32, l as i32]));
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = Vec::from([Vec::from([1, 0, 0]), Vec::from([0, 1, 1]), Vec::from([0, 1, 1])]);
        let expected = Vec::from([Vec::from([0, 0, 0, 0]), Vec::from([1, 1, 2, 2])]);
        assert_eq!(find_farmland(input), expected);
    }

    #[test]
    fn test2() {
        let input = Vec::from([Vec::from([1, 1]), Vec::from([1, 1])]);
        let expected = Vec::from([Vec::from([0, 0, 1, 1])]);
        assert_eq!(find_farmland(input), expected);
    }

    #[test]
    fn test3() {
        let input = Vec::from([Vec::from([0])]);
        let expected: Vec<Vec<i32>> = Vec::from([]);
        assert_eq!(find_farmland(input), expected);
    }

    #[test]
    fn test4() {
        let input = Vec::from([Vec::from([1, 1]), Vec::from([0, 0])]);
        let expected = Vec::from([Vec::from([0, 0, 0, 1])]);
        assert_eq!(find_farmland(input), expected);
    }

    #[test]
    fn test5() {
        let input = Vec::from([Vec::from([1, 1, 0, 0, 0, 1]), Vec::from([1, 1, 0, 0, 0, 0])]);
        let expected = Vec::from([Vec::from([0, 0, 1, 1]), Vec::from([0, 5, 0, 5])]);
        assert_eq!(find_farmland(input), expected);
    }
}
