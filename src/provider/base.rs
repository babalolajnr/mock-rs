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

    fn parse(&self, string: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test;
    impl Base for Test {}

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
}
