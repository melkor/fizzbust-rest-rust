use crate::ports::{FizzBuzzCommand, FizzBuzzer};

pub struct Simple {}

impl FizzBuzzer for Simple {
    fn fizzbuzz(&self, input: FizzBuzzCommand) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let mut n = 1;

        if input.int1 < 1 {
            return Err("int1 is invalid".to_string());
        }

        if input.int2 < 1 {
            return Err("int2 is invalid".to_string());
        }

        if input.limit < 1 || input.limit > 100_000 {
            return Err("limit is invalid".to_string());
        }

        while n <= input.limit {
            if n % input.int1 == 0 && n % input.int2 == 0 {
                result.push(format!("{}{}", input.str1, input.str2));
            } else if n % input.int1 == 0 {
                result.push(input.str1.to_string());
            } else if n % input.int2 == 0 {
                result.push(input.str2.to_string());
            } else {
                result.push(n.to_string());
            }
            n = n + 1;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase {
        given_input: FizzBuzzCommand,
        expected: Result<Vec<String>, String>,
    }

    #[test]
    fn success() {
        let tc = TestCase {
            given_input: FizzBuzzCommand {
                int1: 2,
                int2: 4,
                limit: 10,
                str1: String::from("fizz"),
                str2: String::from("buzz"),
            },
            expected: Ok(vec![
                String::from("1"),
                String::from("fizz"),
                String::from("3"),
                String::from("fizzbuzz"),
                String::from("5"),
                String::from("fizz"),
                String::from("7"),
                String::from("fizzbuzz"),
                String::from("9"),
                String::from("fizz"),
            ]),
        };

        let fizzbuzzer = Simple {};
        let actual = fizzbuzzer.fizzbuzz(tc.given_input);
        assert_eq!(actual, tc.expected);
    }

    #[test]
    fn error_int1() {
        let tc = TestCase {
            given_input: FizzBuzzCommand {
                int1: 0,
                int2: 4,
                limit: 10,
                str1: String::from("fizz"),
                str2: String::from("buzz"),
            },
            expected: Err(String::from("int1 is invalid")),
        };

        let fizzbuzzer = Simple {};
        let actual = fizzbuzzer.fizzbuzz(tc.given_input);
        assert_eq!(actual, tc.expected);
    }

    #[test]
    fn error_int2() {
        let tc = TestCase {
            given_input: FizzBuzzCommand {
                int1: 2,
                int2: 0,
                limit: 10,
                str1: String::from("fizz"),
                str2: String::from("buzz"),
            },
            expected: Err(String::from("int2 is invalid")),
        };

        let fizzbuzzer = Simple {};
        let actual = fizzbuzzer.fizzbuzz(tc.given_input);
        assert_eq!(actual, tc.expected);
    }

    #[test]
    fn error_limit_to_small() {
        let tc = TestCase {
            given_input: FizzBuzzCommand {
                int1: 2,
                int2: 4,
                limit: 0,
                str1: String::from("fizz"),
                str2: String::from("buzz"),
            },
            expected: Err(String::from("limit is invalid")),
        };

        let fizzbuzzer = Simple {};
        let actual = fizzbuzzer.fizzbuzz(tc.given_input);
        assert_eq!(actual, tc.expected);
    }

    #[test]
    fn error_limit_to_big() {
        let tc = TestCase {
            given_input: FizzBuzzCommand {
                int1: 2,
                int2: 4,
                limit: 200_000,
                str1: String::from("fizz"),
                str2: String::from("buzz"),
            },
            expected: Err(String::from("limit is invalid")),
        };

        let fizzbuzzer = Simple {};
        let actual = fizzbuzzer.fizzbuzz(tc.given_input);
        assert_eq!(actual, tc.expected);
    }
}
