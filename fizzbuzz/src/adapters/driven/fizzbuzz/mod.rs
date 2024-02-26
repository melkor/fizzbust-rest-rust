pub mod fizzbuzz {

    pub fn fizzbuzz (int1: i32, int2: i32, limit: i32, str1: String, str2: String ) -> Vec<String> {
        let mut result = Vec::new();
        let mut n = 1;

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

        result
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn success() {
            let truc = fizzbuzz(2, 4, 10, String::from("fizz"), String::from("buzz"));
            let expected = vec!["1", "fizz", "3", "fizzbuzz", "5", "fizz", "7", "fizzbuzz", "9", "fizz"];
            assert_eq!(truc, expected)
        }
    }
}
