use rand::{thread_rng, Rng};

pub trait Base {
    /// Returns a index from array/vector
    fn generate_random_index<T>(&self, arr: &[T]) -> usize {
        let random_index = thread_rng().gen_range(0..arr.len());
        random_index
    }

    /// Returns a random element an array/vector
    fn random_element<'a, T: ?Sized>(&self, arr: &'a Vec<&'a T>) -> &'a T {
        let random_index = self.generate_random_index(&arr);
        &arr[random_index]
    }

    /// Replaces all occurences of '#'s with a random number
    fn numerify(&self, string: Option<&str>) -> String {
        let string = string.unwrap_or("####");

        let charset: &[u8] = b"0123456789";
        let mut rng = rand::thread_rng();

        let random_string = string
            .chars()
            .map(|i| {
                if i == '#' {
                    let idx = rng.gen_range(0..charset.len());
                    charset[idx] as char
                } else {
                    i
                }
            })
            .collect();

        random_string
    }

    fn call_method(&self, string: &str) -> Result<String, String>;

    ///Replaces hash signs ('#') and question marks ('?') with random numbers and letters
    /// An asterisk ('*') is replaced with either a random number or a random letter
    fn bothify(&self, string: Option<&str>) -> String {
        let string = string.unwrap_or("## ??");

        let letterset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let numberset: &[u8] = b"0123456789";
        let mut rng = rand::thread_rng();

        let random_string = string
            .chars()
            .map(|i| {
                if i == '#' {
                    let idx = rng.gen_range(0..numberset.len());
                    numberset[idx] as char
                } else if i == '?' {
                    let idx = rng.gen_range(0..letterset.len());
                    letterset[idx] as char
                } else if i == '*' {
                    let charset = vec![numberset, letterset];
                    let random_set = self.random_element(&charset);
                    let idx = rng.gen_range(0..random_set.len());
                    random_set[idx] as char
                } else {
                    i
                }
            })
            .collect();

        random_string
    }

    /// Get random digit
    fn random_digit(&self) -> u8 {
        let charset: &[u8] = b"0123456789";
        let mut rng = rand::thread_rng();

        let idx = rng.gen_range(0..charset.len());
        charset[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test;
    impl Base for Test {
        fn call_method(&self, string: &str) -> Result<String, String> {
            match string {
                "first_name_male" => Ok(self.first_name_male().to_string()),
                "last_name" => Ok(self.last_name().to_string()),
                _ => Err(format!("Method '{}' not found", string)),
            }
        }
    }

    impl Test {
        pub fn first_name_male<'a>(&self) -> &'a str {
            "John"
        }

        pub fn last_name<'a>(&self) -> &'a str {
            "Doe"
        }
    }

    #[test]
    fn numerify_works() {
        let string = "####";
        let test = Test {};

        let result = test.numerify(Some(string));
        assert_eq!(result.len(), string.len());
    }

    #[test]
    fn numerify_works_with_no_string() {
        let string = None;
        let test = Test {};

        let result = test.numerify(string);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn numerify_works_with_string_of_hashes_mixed_with_other_chars() {
        let string = "Hello #####";
        let test = Test {};

        let result = test.numerify(Some(string));
        assert_eq!(result.len(), string.len());
    }

    #[test]
    fn generate_random_index_works() {
        let arr = vec![1, 2, 3, 4, 5];
        let test = Test {};

        let result = test.generate_random_index(&arr);
        assert!(result < arr.len());
    }

    #[test]
    fn random_element_works() {
        let arr = vec!["a", "b", "c", "d", "e"];
        let test = Test {};

        let result = test.random_element(&arr);
        assert!(arr.contains(&result));
    }

    #[test]
    fn bothify() {
        let string = "## ??";

        let test = Test {};
        let result = test.bothify(Some(string));
        println!("{}", result);
    }
}
