pub trait FizzBuzzer {
    fn fizzbuzz (&self, int1: i32, int2: i32, limit: i32, str1: String, str2: String ) -> Result<Vec<String>, String>; 
}
