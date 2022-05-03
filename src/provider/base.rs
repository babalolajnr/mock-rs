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

    /// Replaces tokens ('{{ tokenName }}') with the result from the token method call
    fn parse(&self, string: &str) -> String {
        let mut result = String::new();

        let remove_opening_braces: String = string.split("{{").collect::<String>();
        let tokens = remove_opening_braces.split("}}").collect::<String>();

        let methods: Vec<&str> = tokens.split(" ").collect::<Vec<&str>>();
        let methods: Vec<&&str> = methods.iter().filter(|&x| x != &"").collect();

        for i in 0..methods.len() {
            let token_value = self.call_method(methods[i]);

            if i == methods.len() - 1 && methods.len() > 1 {
                result.push_str(&(" ".to_string() + &token_value.unwrap()));
            } else {
                result.push_str(&token_value.unwrap());
            }
        }

        result
    }

    fn call_method(&self, string: &str) -> Result<String, String>;

    ///Replaces hash signs ('#') and question marks ('?') with random numbers and letters
    /// An asterisk ('*') is replaced with either a random number or a random letter
    fn bothify(&self, string: Option<&str>) -> String {
        let string = string.unwrap_or("## ??");

        let charset: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();

        let random_string = string
            .chars()
            .map(|i| {
                if i == '#' {
                    let idx = rng.gen_range(0..charset.len());
                    charset[idx] as char
                } else if i == '?' {
                    let idx = rng.gen_range(0..charset.len());
                    charset[idx] as char
                } else if i == '*' {
                    let idx = rng.gen_range(0..charset.len());
                    charset[idx] as char
                } else {
                    i
                }
            })
            .collect();

        random_string
    }

    // fn replace_wildcard<T>(
    //     &self,
    //     string: &str,
    //     wildcard: Option<&str>,
    //     callback: Option<T>,
    // ) -> String
    // where
    //     T: Fn() -> str,
    // {
    //     let string = string.to_string();
    //     let wildcard = wildcard.unwrap_or("#");
    //     let callback = callback.unwrap_or((|| self.random_digit()) as T);

    //     let mut result = String::new();

    //     let mut chars = string.chars();
    //     let mut last_char = chars.next();

    //     for c in chars {
    //         if c == wildcard.chars().next().unwrap() {
    //             if last_char == Some(wildcard.chars().next().unwrap()) {
    //                 result.push_str(&callback(wildcard));
    //             } else {
    //                 result.push_str(&wildcard);
    //             } 
    //         } else {
    //             result.push(c);
    //         }

    //         last_char = Some(c);
    //     }

    //     result
    // }

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
    fn parse_works() {
        let string = "{{first_name_male}}";

        let test = Test {};
        let result = test.parse(string);
        assert_eq!("John".to_string(), result)
    }

    #[test]
    fn parse_works_without_leading_spaces() {
        let string = "{{ first_name_male }}";

        let test = Test {};
        let result = test.parse(string);
        assert_eq!("John".to_string(), result)
    }

    #[test]
    fn parse_multiple_format_works() {
        let string = "{{ first_name_male }} {{ last_name }}";

        let test = Test {};
        let result = test.parse(string);
        assert_eq!("John Doe".to_string(), result)
    }

    #[test]
    fn parse_multiple_format_works_without_spaces_in_between() {
        let string = "{{ first_name_male }}{{ last_name }}";

        let test = Test {};
        let result = test.parse(string);
        assert_eq!("John Doe".to_string(), result)
    }
}
