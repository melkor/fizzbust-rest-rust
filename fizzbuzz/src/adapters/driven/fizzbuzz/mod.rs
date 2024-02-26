pub mod fizzbuzz {

    pub fn fizzbuzz (int1: i32, int2: i32, limit: i32, str1: String, str2: String ) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let mut n = 1;

        if int1 < 1 {
            return Err("int1 is invalid".to_string())
        }

        if int2 < 1 {
            return Err("int2 is invalid".to_string())
        }

        if limit < 1 || limit > 100_000 {
            return Err("limit is invalid".to_string())
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

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn success() {
            let expected = vec!["1", "fizz", "3", "fizzbuzz", "5", "fizz", "7", "fizzbuzz", "9", "fizz"];
            match fizzbuzz(2, 4, 10, String::from("fizz"), String::from("buzz")) {
                Ok(actual) => assert_eq!(actual, expected),
                Err(err) => assert_eq!(err, "")
            }
        }

        #[test]
        fn error_int1() {
            let expected: Vec<String> = Vec::new();
            match fizzbuzz(0, 4, 10, String::from("fizz"), String::from("buzz")) {
                Ok(actual) => assert_eq!(actual, expected),
                Err(err) => assert_eq!(err, "int1 is invalid")
            }
        }

        #[test]
        fn error_int2() {
            let expected: Vec<String> = Vec::new();
            match fizzbuzz(2, 0, 10, String::from("fizz"), String::from("buzz")) {
                Ok(actual) => assert_eq!(actual, expected),
                Err(err) => assert_eq!(err, "int2 is invalid")
            }
        }

        #[test]
        fn error_limit_to_small() {
            let expected: Vec<String> = Vec::new();
            match fizzbuzz(2, 4, 0, String::from("fizz"), String::from("buzz")) {
                Ok(actual) => assert_eq!(actual, expected),
                Err(err) => assert_eq!(err, "limit is invalid")
            }
        }

        #[test]
        fn error_limit_to_big() {
            let expected: Vec<String> = Vec::new();
            match fizzbuzz(2, 4, 200_000, String::from("fizz"), String::from("buzz")) {
                Ok(actual) => assert_eq!(actual, expected),
                Err(err) => assert_eq!(err, "limit is invalid")
            }
        }
    }
}
