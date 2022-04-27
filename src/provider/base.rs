use rand::{thread_rng, Rng};
pub trait Base {
    /// Returns a index from array/vector
    fn generate_random_index<T>(arr: &[T]) -> usize {
        let random_index = thread_rng().gen_range(1..arr.len());
        random_index
    }

    /// Returns a random element an array/vector
    fn random_element<'a, T: ?Sized>(arr: &'a Vec<&'a T>) -> &'a T {
        let random_index = Self::generate_random_index(&arr);
        &arr[random_index]
    }

    /// Generates a random string that matches the
    /// length of the given string containing "#"s
    fn numerify(string: Option<&str>) -> String {
        let string = string.unwrap_or("###");

        let charset: &[u8] = b"0123456789";
        let mut rng = rand::thread_rng();

        let random_string: String = (0..string.len())
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect();

        random_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn numerify_works() {

        let string = "####";
        struct Test;
        impl Base for Test {}

        let result = Test::numerify(Some(string));
        assert_eq!(result.len(), string.len());
    }

    #[test]
    fn numerify_works_with_no_string() {

        let string = None;
        struct Test;
        impl Base for Test {}

        let result = Test::numerify(string);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn generate_random_index_works() {
        let arr = vec![1, 2, 3, 4, 5];
        struct Test;
        impl Base for Test {}

        let result = Test::generate_random_index(&arr);
        assert!(result < arr.len());
    }

    #[test]
    fn random_element_works() {
        let arr = vec!["a", "b", "c", "d", "e"];
        struct Test;
        impl Base for Test {}

        let result = Test::random_element(&arr);
        assert!(arr.contains(&result));
    }
}
