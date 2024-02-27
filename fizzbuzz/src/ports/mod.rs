pub trait FizzBuzzer {
    fn fizzbuzz(&self, input: FizzBuzzCommand) -> Result<Vec<String>, String>;
}

pub struct FizzBuzzCommand {
    pub int1: i32,
    pub int2: i32,
    pub limit: i32,
    pub str1: String,
    pub str2: String,
}
