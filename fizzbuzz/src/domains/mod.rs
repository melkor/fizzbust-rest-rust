use crate::ports::{FizzBuzzer, FizzBuzzCommand};

pub fn get_fizzbuzz(fizzbuzz: &impl FizzBuzzer) -> Result<Vec<String>, String>{
    fizzbuzz.fizzbuzz(
        FizzBuzzCommand{
            int1: 3,
            int2: 5,
            limit: 10,
            str1: "str1".to_string(), 
            str2: "str2".to_string(), 
        },
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mytest() {
        let mut mock = crate::ports::MockFizzBuzzer::new();
        let expected: Result<Vec<String>, String> = Ok(vec!["".to_string()]);
        mock.expect_fizzbuzz()
            .times(1)
            .return_const(expected);
        match mock.fizzbuzz(FizzBuzzCommand{
                int1: 3,
                int2: 5,
                limit: 10,
                str1: "str1".to_string(), 
                str2: "str2".to_string(), 
            },
        ) {
            Ok(actual) => assert_eq!(actual, vec![""]),
            Err(err) => assert_eq!(err, "".to_string()),
        }
    }
}
