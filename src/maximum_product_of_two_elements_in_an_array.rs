// https://leetcode.com/problems/day-of-the-week/
// https://leetcode.com/problems/add-binary/submissions/936487266/

use chrono::NaiveDate;

pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
    NaiveDate::from_ymd(2015, 3, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(day_of_the_week(31,8,2019), "Saturday");
    }

    #[test]
    fn test2() {
        assert_eq!(day_of_the_week(18,7,1999), "Sunday");
    }

    #[test]
    fn test3() {
        assert_eq!(day_of_the_week(15,8,1993), "Sunday");
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
