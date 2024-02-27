use crate::ports::{FizzBuzzCommand, FizzBuzzer};

struct Simple {}

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
        expected_result: Vec<String>,
        expected_error: String,
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
            expected_result: vec![
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
            ],
            expected_error: String::new(),
        };

        let acutal_fizzbuzzer = Simple {};
        match acutal_fizzbuzzer.fizzbuzz(tc.given_input) {
            Ok(actual) => assert_eq!(actual, tc.expected_result),
            Err(err) => assert_eq!(err, tc.expected_error),
        }
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
            expected_result: Vec::new(),
            expected_error: String::from("int1 is invalid"),
        };

        let acutal_fizzbuzzer = Simple {};
        match acutal_fizzbuzzer.fizzbuzz(tc.given_input) {
            Ok(actual) => assert_eq!(actual, tc.expected_result),
            Err(err) => assert_eq!(err, tc.expected_error),
        }
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
            expected_result: Vec::new(),
            expected_error: String::from("int2 is invalid"),
        };

        let acutal_fizzbuzzer = Simple {};
        match acutal_fizzbuzzer.fizzbuzz(tc.given_input) {
            Ok(actual) => assert_eq!(actual, tc.expected_result),
            Err(err) => assert_eq!(err, tc.expected_error),
        }
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
            expected_result: Vec::new(),
            expected_error: String::from("limit is invalid"),
        };

        let acutal_fizzbuzzer = Simple {};
        match acutal_fizzbuzzer.fizzbuzz(tc.given_input) {
            Ok(actual) => assert_eq!(actual, tc.expected_result),
            Err(err) => assert_eq!(err, tc.expected_error),
        }
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
            expected_result: Vec::new(),
            expected_error: String::from("limit is invalid"),
        };

        let acutal_fizzbuzzer = Simple {};
        match acutal_fizzbuzzer.fizzbuzz(tc.given_input) {
            Ok(actual) => assert_eq!(actual, tc.expected_result),
            Err(err) => assert_eq!(err, tc.expected_error),
        }
    }
}
