use std::io;
use std::str::FromStr;

fn main() {
    // prime();
    // decomposition();
    // fibonacci_numbers(1_000_000);
    triangle();
}

fn triangle() {
    if is_valid_triangle(get_u64_from_user(), get_u64_from_user(), get_u64_from_user()) {
        println!("Valid triangle")
    } else {
        println!("Not valid triangle")
    }
}

fn is_valid_triangle(a: u64, b: u64, c: u64) -> bool {
    return a + b > c && a + c > b && b + c > a;
}

fn fibonacci_numbers(max: u64) -> Vec<u64> {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(1);
    print!("1,1,");
    let mut i = 2;
    let mut val = vec[i - 2] + vec[i - 1];
    while val < max {
        vec.push(val);
        print!("{val},");
        i += 1;
        val = vec[i - 2] + vec[i - 1];
    }
    return vec;
}

fn decomposition() {
    let vec = decomposition_of_a_number(get_u64_from_user());
    for item in vec {
        println!("|{item}");
    }
}

fn decomposition_of_a_number(mut number: u64) -> Vec<u64> {
    let mut i = 2;
    let mut vec = Vec::new();
    while number > 1 {
        if number % i == 0 {
            println!("{number:8} | {i}");
            vec.push(i);
            number /= i;
        } else {
            i += 1;
        }
    }
    println!("{number:8}");
    return vec;
}

fn prime() {
    // 215647
    let number = get_u64_from_user();
    if is_prime(number) {
        println!("{number} is prime")
    } else {
        println!("{number} is not prime")
    }
}

fn get_u64_from_user() -> u64 {
    println!("Insert number:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().parse()
        .expect("Can't parse to u64");
}

fn is_prime(number: u64) -> bool {
    if number == 1 {
        return false;
    }
    // let sqrt = (number as f64).sqrt() as u64;
    // for i in 2..=sqrt {
    //     if number % i == 0 {
    //         return false;
    //     }
    // }
    let mut i = 2;
    while i * i <= number {
        if number % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_prime_1() {
        assert_eq!(is_prime(1), false);
    }

    #[test]
    fn test_is_prime_2() {
        assert_eq!(is_prime(2), true);
    }

    #[test]
    fn test_is_prime_3() {
        assert_eq!(is_prime(3), true);
    }

    #[test]
    fn test_is_prime_4() {
        assert_eq!(is_prime(4), false);
    }

    #[test]
    fn test_is_prime_5() {
        assert_eq!(is_prime(5), true);
    }

    #[test]
    fn test_is_prime_6() {
        assert_eq!(is_prime(6), false);
    }

    #[test]
    fn test_is_prime_97() {
        assert_eq!(is_prime(97), true);
    }

    #[test]
    fn test_decomposition_of_a_number_60() {
        assert_eq!(decomposition_of_a_number(60), vec![2, 2, 3, 5]);
    }

    #[test]
    fn test_decomposition_of_a_number_62() {
        assert_eq!(decomposition_of_a_number(62), vec![2, 31]);
    }

    #[test]
    fn test_decomposition_of_a_number_20() {
        assert_eq!(decomposition_of_a_number(20), vec![2, 2, 5]);
    }

    #[test]
    fn test_decomposition_of_a_number_2160() {
        assert_eq!(decomposition_of_a_number(2160), vec![2, 2, 2, 2, 3, 3, 3, 5]);
    }

    #[test]
    fn test_fibonacci_numbers_until_100() {
        assert_eq!(fibonacci_numbers(100), vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    }

    #[test]
    fn test_is_valid_triangle_1_2_3() {
        assert_eq!(is_valid_triangle(1, 2, 3), false);
    }

    #[test]
    fn test_is_valid_triangle_2_2_3() {
        assert_eq!(is_valid_triangle(2, 2, 3), true);
    }

    #[test]
    fn test_is_valid_triangle_5_2_2() {
        assert_eq!(is_valid_triangle(5, 2, 2), false);
    }

    #[test]
    fn test_is_valid_triangle_2_5_2() {
        assert_eq!(is_valid_triangle(2, 5, 2), false);
    }

    #[test]
    fn test_is_valid_triangle_3_3_3() {
        assert_eq!(is_valid_triangle(3, 3, 3), true);
    }
}
