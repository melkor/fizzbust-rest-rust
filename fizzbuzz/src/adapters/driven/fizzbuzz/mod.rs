pub mod fizzbuzz {

    pub fn fizzbuzz (int1: i32, int2: i32, limit: i32, str1: String, str2: String ) -> String {
        let mut result = String::new();
        let mut n = 1;

        while n <= limit {
            if n % int1 == 0 && n % int2 == 0 {
                result = format!("{result}{str1}{str2}-");
            } else if n % int1 == 0 {
                result = format!("{result}{str1}-");
            } else if n % int2 == 0 {
                result = format!("{result}{str2}-");
            } else {
                let str_int2 = int2.to_string();
                result = format!("{result}{str_int2}-");
            }
            n = n + 1;
        }

        result
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn success() {
            let truc = fizzbuzz(2, 4, 10, String::from("fizz"), String::from("buzz"));
            assert_eq!(truc, String::new())
        }
    }
}
