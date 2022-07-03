use std::{collections::HashMap, fmt::Display};

use rand::{thread_rng, Rng};

pub trait BaseTrait {
    /// Returns a index from array/vector
    fn random_index<T>(arr: &[T]) -> usize {
        let random_index = thread_rng().gen_range(0..arr.len());
        random_index
    }

    /// Returns a random element an array/vector
    fn random_element<'a, T>(arr: &'a Vec<&T>) -> &'a T
    where
        T: ?Sized,
    {
        let random_index = Self::random_index(&arr);
        &arr[random_index]
    }

    /// Replaces all occurences of '#'s with a random number
    fn numerify(string: Option<&str>) -> String {
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

    ///Replaces hash signs ('#') and question marks ('?') with random numbers and letters.
    ///An asterisk ('*') is replaced with either a random number or a random letter
    fn bothify(string: Option<&str>) -> String {
        let string = string.unwrap_or("## ??");

        let letterset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
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
                    let random_set = Self::random_element(&charset);
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
    fn random_digit() -> u8 {
        let charset: &[u8] = b"0123456789";
        let mut rng = rand::thread_rng();

        let idx = rng.gen_range(0..charset.len());
        charset[idx]
    }

    /// Get random letter
    fn random_letter() -> char {
        let charset_string = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
        let charset_vec: Vec<char> = charset_string.chars().collect();

        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..charset_vec.len());

        charset_vec[idx]
    }

    // Get a random Key from a HashMap
    fn random_key<'a, K, V>(hash_map: &HashMap<K, V>) -> K
    where
        K: 'a + Display + ToOwned<Owned = K>,
    {
        let keys: Vec<&K> = hash_map.keys().into_iter().map(|k| k).collect();
        let random_key = Self::random_element(&keys);
        random_key.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test;
    impl BaseTrait for Test {}

    #[test]
    fn numerify_works() {
        let string = "####";

        let result = Test::numerify(Some(string));
        assert_eq!(result.len(), string.len());
    }

    #[test]
    fn numerify_works_with_no_string() {
        let string = None;

        let result = Test::numerify(string);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn numerify_works_with_string_of_hashes_mixed_with_other_chars() {
        let string = "Hello #####";

        let result = Test::numerify(Some(string));
        assert_eq!(result.len(), string.len());
    }

    #[test]
    fn generate_random_index_works() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = Test::random_index(&arr);
        assert!(result < arr.len());
    }

    #[test]
    fn random_element_works() {
        let arr = vec!["a", "b", "c", "d", "e"];
        let result = Test::random_element(&arr);
        assert!(arr.contains(&result));
    }

    #[test]
    fn bothify() {
        let string = "?####";
        let result = Test::bothify(Some(string));
        assert_eq!(result.len(), string.len());
    }

    #[test]
    fn random_key_for_str_slice() {
        let mut hash_map: HashMap<&str, &str> = HashMap::new();
        hash_map.insert("a", "b");
        hash_map.insert("c", "d");
        hash_map.insert("e", "f");

        let result = Test::random_key(&hash_map);

        assert!(hash_map.contains_key(&result));
        println!("{}", result);
    }

    #[test]
    fn random_key_for_string() {
        let mut hash_map: HashMap<String, &str> = HashMap::new();
        hash_map.insert("a".to_string(), "b");
        hash_map.insert("c".to_string(), "d");
        hash_map.insert("e".to_string(), "f");

        let result = Test::random_key(&hash_map);

        assert!(hash_map.contains_key(&result));
        println!("{}", result);
    }

    #[test]
    fn random_key_for_numeric_type() {
        let mut hash_map: HashMap<usize, &str> = HashMap::new();
        hash_map.insert(1, "b");
        hash_map.insert(2, "d");
        hash_map.insert(3, "f");

        let result = Test::random_key(&hash_map);

        assert!(hash_map.contains_key(&result));
        println!("{}", result);
    }

    #[test]
    fn get_random_letter() {
        let random_letter = Test::random_letter();
        println!("{}", random_letter);

        assert!(random_letter.is_alphabetic());
    }
}
