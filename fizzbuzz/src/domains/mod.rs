/*
use crate::ports::{FizzBuzzer, FizzBuzzCommand};

pub fn get_fizzbuzz(fizzbuzzadapter: &impl FizzBuzzer) -> Result<Vec<String>, String>{
    fizzbuzzadapter.fizzbuzz(
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
        let fizzbuzz_return: Result<Vec<String>, String> = Ok(vec!["".to_string()]);
        mock.expect_fizzbuzz()
            .times(1)
            .return_const(fizzbuzz_return);

        let actual = get_fizzbuzz(&mock);

        let expected: Result<Vec<String>, String> = Ok(vec!["".to_string()]);
        assert_eq!(actual, expected);
    }
}
*/
