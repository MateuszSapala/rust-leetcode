// https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/
// https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/submissions/938055979/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..group_sizes.len() {
            let size = group_sizes[i];
            let group_in_map = map.get(&size);
            match group_in_map {
                Some(x) => {
                    let mut y = (*x).clone();
                    y.push(i as i32);
                    map.insert(size, y);
                }
                None => {
                    map.insert(size, Vec::from([i as i32]));
                }
            }
        }

        let mut result: Vec<Vec<i32>> = Vec::new();
        for (key, value) in map {
            let mut val = value.clone();
            while val.len() > 0 {
                let mut vec: Vec<i32> = Vec::new();
                for _ in 0..key {
                    vec.push(val.pop().unwrap());
                }
                result.push(vec);
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
        verify(Vec::from([3, 3, 3, 3, 3, 1, 3]));
    }

    #[test]
    fn test2() {
        verify(Vec::from([2, 1, 3, 3, 3, 2]));
    }

    fn verify(input: Vec<i32>) {
        let v = input.clone();
        let result = Solution::group_the_people(input);
        println!("result: {:?}", result);
        let mut people_assigned = 0;
        for group in result {
            let group_size = group.len();
            for person in group {
                if v[person as usize] != group_size as i32 {
                    panic!("Incorrect result")
                }
                people_assigned += 1;
            }
        }
        if people_assigned != v.len() {
            panic!("Assigned {people_assigned} people instead of {}", v.len())
        }
    }
}
