use crate::ports::FizzBuzzer;

struct Simple {}

impl crate::ports::FizzBuzzer for Simple {
    fn fizzbuzz(
        &self,
        int1: i32,
        int2: i32,
        limit: i32,
        str1: String,
        str2: String,
    ) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let mut n = 1;

        if int1 < 1 {
            return Err("int1 is invalid".to_string());
        }

        if int2 < 1 {
            return Err("int2 is invalid".to_string());
        }

        if limit < 1 || limit > 100_000 {
            return Err("limit is invalid".to_string());
        }

        while n <= limit {
            if n % int1 == 0 && n % int2 == 0 {
                result.push(format!("{str1}{str2}"));
            } else if n % int1 == 0 {
                result.push(format!("{str1}"));
            } else if n % int2 == 0 {
                result.push(format!("{str2}"));
            } else {
                let str_n = n.to_string();
                result.push(format!("{str_n}"));
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
        //name: String,
        int1: i32,
        int2: i32,
        limit: i32,
        str1: String,
        str2: String,
        expected_result: Vec<String>,
        expected_error: String,
    }

    #[test]
    fn success() {
        let tc = TestCase {
            int1: 2,
            int2: 4,
            limit: 10,
            str1: String::from("fizz"),
            str2: String::from("buzz"),
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
        match acutal_fizzbuzzer.fizzbuzz(tc.int1, tc.int2, tc.limit, tc.str1, tc.str2) {
            Ok(actual) => assert_eq!(actual, tc.expected_result),
            Err(err) => assert_eq!(err, tc.expected_error),
        }
    }

    #[test]
    fn error_int1() {
        let tc = TestCase {
            int1: 0,
            int2: 4,
            limit: 10,
            str1: String::from("fizz"),
            str2: String::from("buzz"),
            expected_result: Vec::new(),
            expected_error: String::from("int1 is invalid"),
        };

        let acutal_fizzbuzzer = Simple {};
        match acutal_fizzbuzzer.fizzbuzz(tc.int1, tc.int2, tc.limit, tc.str1, tc.str2) {
            Ok(actual) => assert_eq!(actual, tc.expected_result),
            Err(err) => assert_eq!(err, tc.expected_error),
        }
    }

    #[test]
    fn error_int2() {
        let tc = TestCase {
            int1: 2,
            int2: 0,
            limit: 10,
            str1: String::from("fizz"),
            str2: String::from("buzz"),
            expected_result: Vec::new(),
            expected_error: String::from("int2 is invalid"),
        };

        let acutal_fizzbuzzer = Simple {};
        match acutal_fizzbuzzer.fizzbuzz(tc.int1, tc.int2, tc.limit, tc.str1, tc.str2) {
            Ok(actual) => assert_eq!(actual, tc.expected_result),
            Err(err) => assert_eq!(err, tc.expected_error),
        }
    }

    #[test]
    fn error_limit_to_small() {
        let tc = TestCase {
            int1: 2,
            int2: 4,
            limit: 0,
            str1: String::from("fizz"),
            str2: String::from("buzz"),
            expected_result: Vec::new(),
            expected_error: String::from("limit is invalid"),
        };

        let acutal_fizzbuzzer = Simple {};
        match acutal_fizzbuzzer.fizzbuzz(tc.int1, tc.int2, tc.limit, tc.str1, tc.str2) {
            Ok(actual) => assert_eq!(actual, tc.expected_result),
            Err(err) => assert_eq!(err, tc.expected_error),
        }
    }

    #[test]
    fn error_limit_to_big() {
        let tc = TestCase {
            int1: 2,
            int2: 4,
            limit: 200_000,
            str1: String::from("fizz"),
            str2: String::from("buzz"),
            expected_result: Vec::new(),
            expected_error: String::from("limit is invalid"),
        };

        let acutal_fizzbuzzer = Simple {};
        match acutal_fizzbuzzer.fizzbuzz(tc.int1, tc.int2, tc.limit, tc.str1, tc.str2) {
            Ok(actual) => assert_eq!(actual, tc.expected_result),
            Err(err) => assert_eq!(err, tc.expected_error),
        }
    }
}
