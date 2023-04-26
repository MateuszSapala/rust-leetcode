// https://leetcode.com/problems/add-binary/
// https://leetcode.com/problems/add-binary/submissions/936487266/

pub fn add_binary(a: String, b: String) -> String {
    let last_a = a.len() - 1;
    let last_b = b.len() - 1;
    let last_idx = if last_a >= last_b { a.len() } else { b.len() };
    let mut str = String::new();
    let mut transferred: u32 = 0;
    for i in 0..last_idx {
        let mut a = a.chars();
        let mut b = b.chars();
        let val_a = if last_a >= i { a.nth(last_a - i).unwrap().to_digit(2).expect("Unable to parse to u32") } else { 0 };
        let val_b = if last_b >= i { b.nth(last_b - i).unwrap().to_digit(2).expect("Unable to parse to u32") } else { 0 };
        let sum = val_a + val_b + transferred;
        str = (sum % 2).to_string() + &str;
        transferred = sum / 2;
    }
    if transferred > 0 {
        str = transferred.to_string() + &str;
    }
    return str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(add_binary("11".to_owned(), "1".to_owned()), "100");
    }

    #[test]
    fn test2() {
        assert_eq!(add_binary("1010".to_owned(), "1011".to_owned()), "10101");
    }
}

// // Line 3, Char 44: Unable to parse to i64: ParseIntError { kind: PosOverflow } (solution.rs)
// pub fn add_binary(a: String, b: String) -> String {
//     let a = i64::from_str_radix(&a, 2).expect("Unable to parse to i64");
//     let b = i64::from_str_radix(&b, 2).expect("Unable to parse to i64");
//     return format!("{:b}", (a+b));
// }

// // Line 3, Char 44: Unable to parse to i32: ParseIntError { kind: PosOverflow } (solution.rs)
// pub fn add_binary(a: String, b: String) -> String {
//     let a = i32::from_str_radix(&a, 2).expect("Unable to parse to i32");
//     let b = i32::from_str_radix(&b, 2).expect("Unable to parse to i32");
//     return format!("{:b}", (a+b));
// }
