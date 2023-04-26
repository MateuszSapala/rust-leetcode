// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/
// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/submissions/937474540/

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut snd_max = 0;
    for n in nums {
        if n > max {
            snd_max = max;
            max = n;
        } else if n > snd_max {
            snd_max = n;
        }
    }
    return (max - 1) * (snd_max - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(max_product(Vec::from([3, 4, 5, 2])), 12);
    }

    #[test]
    fn test2() {
        assert_eq!(max_product(Vec::from([1, 5, 4, 5])), 16);
    }

    #[test]
    fn test3() {
        assert_eq!(max_product(Vec::from([3, 7])), 12);
    }
}
